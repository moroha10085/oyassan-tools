use anyhow::Result;
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use oyassan::{input, Config, LootBox, PrefImgGenerator, ZipBuilder};
use ringbuf::traits::{Consumer, Observer, Producer, Split};
use ringbuf::SharedRb;
use rodio::{Decoder, OutputStream, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufReader};
use std::sync::mpsc;
use std::thread::Scope;
use std::{thread, time};

#[derive(Debug, Clone, PartialEq, Eq)]
struct EndPaint;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[arg(long, short)]
    advanced: bool,

    #[arg(long, short)]
    use_config: bool,

    #[arg(long)]
    input_path: Option<String>,
}

static NOTICE_SOUND: &str = "./data/notice.mp3";

fn indicator(prefs_len: usize) -> Result<(ProgressBar, ProgressBar)> {
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise} {bar:60.green/blue}] {pos:>4}/{len:4} {msg}")?
        .progress_chars("##>-");

    let multi_bar = MultiProgress::new();
    let gen_bar = {
        let bar = ProgressBar::new(prefs_len as u64);
        bar.set_style(style.clone());
        multi_bar.add(bar)
    };
    let save_bar = {
        let bar = ProgressBar::new(prefs_len as u64);
        bar.set_style(style);
        multi_bar.add(bar)
    };

    Ok((gen_bar, save_bar))
}

fn paint(config: &Config) -> Result<()> {
    let mut zip = ZipBuilder::create().expect("failed to create zip.");
    let out_path = zip.path.clone();

    thread::scope(|s: &Scope<'_, '_>| {
        let input = input(config).expect("failed to get input");
        let (gen_bar, save_bar) =
            indicator(input.prefs.len()).expect("failed to create progress bar");

        let mut generator = PrefImgGenerator::new(config.resolution.as_size());

        let zip_queue = SharedRb::new(8);
        let (mut tx, mut rx) = zip_queue.split();
        let (state_tx, state_rx) = mpsc::channel();

        s.spawn(move || {
            let mut num_of_pref_map = HashMap::new();
            let mut save_msg = LootBox::new(vec![
                String::from("おやっさんは丁寧に塗っている。"),
                String::from("おやっさんはふちを気をつけて塗っている。"),
                String::from("おやっさんは細かい部分に目を凝らしている。"),
                String::from("おやっさんは無心で塗っている。"),
                String::from("おやっさんは塗りむらが出ないようにしている。"),
            ]);

            for (idx, pref) in input.prefs.into_iter().enumerate() {
                gen_bar.inc(1);
                gen_bar.set_message(save_msg.roll());

                let num_of_pref = *num_of_pref_map.get(&pref).unwrap_or(&0);
                num_of_pref_map.insert(pref.clone(), num_of_pref + 1);

                let tint_color = input.colors[num_of_pref % input.colors.len()];
                generator.overlay(&pref, &tint_color);

                while tx.is_full() {
                    gen_bar.set_message("おやっさんはボブの仕事を待っている。");
                }

                tx.try_push((generator.get_img(), format!("{:06}.png", idx)))
                    .expect("failed to push to queue");
            }

            state_tx.send(EndPaint).expect("failed to send state");
            gen_bar.finish();
        });

        s.spawn(move || {
            let mut done_generated = false;
            let mut gen_msg = LootBox::new(vec![
                String::from("ボブはzipファイルに画像をそっとしまっている。"),
                String::from("ボブはおやっさんの作品を大事にzipファイルに入れている。"),
                String::from("ボブはおやっさんの作品に感動している。"),
                String::from("ボブは画像にシワが入らないように気をつけている"),
                String::from("ボブはおやっさんに褒められている"),
                String::from("ボブはzipファイルにしまっている"),
            ]);

            loop {
                save_bar.set_message(gen_msg.roll());
                save_bar.inc(1);

                if state_rx.try_recv().is_ok() {
                    done_generated = true;
                }

                if done_generated && rx.is_empty() {
                    break;
                }
                let begin_wait = time::Instant::now();

                while rx.is_empty() {
                    let wait_elapsed_millis = begin_wait.elapsed().as_millis();
                    if wait_elapsed_millis > 100 {
                        save_bar.set_message("ボブはおやっさんの作業を待っている...。");
                    }
                }

                let (img, file_dir) = rx.try_pop().expect("failed to pop from queue");
                zip.add_png(img, file_dir)
                    .expect("failed to add png to zip.");
            }

            zip.zip.finish().expect("failed to finish zip");
            save_bar.finish();
        });
    });

    println!(
        "おやっさん「あんたの依頼品は`{}`に置いといたからな。」",
        out_path.to_str().expect("failed to convert path to str")
    );

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut config = Config::wizard(args.advanced, args.use_config)?;
    if args.use_config {
        let c = Config::from_file()?;
        config = Config {
            input_path: config.input_path,
            ..c
        };
    }

    paint(&config)?;

    if config.play_notification_sound {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = File::open(NOTICE_SOUND)?;
        let file = BufReader::new(file);
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples())?;
    }

    println!("閉じるにはEnterを押してください。");
    let mut _buf = String::new();
    stdin().read_line(&mut _buf)?;

    Ok(())
}
