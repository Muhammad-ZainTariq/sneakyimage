mod steganography;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a message into an image
    #[command(verbatim_doc_comment)]
    Enc {
        /// The path to the input image
        input: PathBuf,

        /// The secret message to encode
        message: String,

        /// The path for the output image
        output: PathBuf,
    },
    /// Decode a message from an image
    Dec {
        /// The path to the encoded image
        input: PathBuf,
    },
}

fn print_art() {
    let art = r#"
               ...
             ;::::;
           ;::::; :;
         ;:::::'   :;
        ;:::::;     ;.
       ,:::::'       ;           OOO\
       ::::::;       ;          OOOOO\
       ;:::::;       ;         OOOOOOOO
      ,;::::::;     ;'         / OOOOOOO
    ;:::::::::`. ,,,;.        /  / DOOOOOO
  .';:::::::::::::::::;,     /  /     DOOOO
 ,::::::;::::::;;;;::::;,   /  /        DOOO
;`::::::`'::::::;;;::::: ,#/  /          DOOO
:`:::::::`;::::::;;::: ;::#  /            DOOO
::`:::::::`;:::::::: ;::::# /              DOO
`:`:::::::`;:::::: ;::::::#/               DOO
 :::`:::::::`;; ;:::::::::##                OO
 ::::`:::::::`;::::::::;:::#                OO
 `:::::`::::::::::::;'`:;::#                O
  `:::::`::::::::;' /  / `:#
   ::::::`:::::;'  /  /   `#
"#;
    println!("{}", art);
    println!("--- SneakyImage ---");
}

fn main() -> Result<()> {
    print_art();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Enc {
            input,
            message,
            output,
        } => {
            println!(
                "Encoding message in '{}' and saving to '{}'",
                input.display(),
                output.display()
            );
            steganography::encode(input, message, output)?;
            println!("Encoding complete!");
        }
        Commands::Dec { input } => {
            println!("Decoding message from '{}'", input.display());
            let message = steganography::decode(input)?;
            println!("The secret message is: {}", message);
        }
    }

    Ok(())
} 