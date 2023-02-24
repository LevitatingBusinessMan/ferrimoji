use std::fs;
use libvips::{ops, VipsImage, VipsApp};
use anyhow::{Result, Error};

static SCALEFACTOR: f64 = 8.0;

fn main() -> Result<(), Error> {   
    let vips = VipsApp::new("Ferrimoji", false).expect("Could not initiate libvips");
    let ferris = VipsImage::new_from_file("images/ferris.png")?;

    //let ferris = VipsImage::new_from_file("images/ferris.png")?;

    fs::create_dir_all("out")?;


    for emoji_path in fs::read_dir("images/emojis")? {


        /*
        I manually strip the svg because rendering it causes antialiasing to create colors we can't pick out later.
        I could fix this by manually replacing the pixels (checking if they come close to #FFCC4D) but I don't have that kind of time.
        */

        let emoji_path = emoji_path?.path();
        //let emoji_path = "images/emojis/1f605.svg";

        let emoji_svg = fs::read_to_string(&emoji_path)?;
        let mut parts = emoji_svg.split(">").collect::<Vec<&str>>();
        
        // Remove the yellow circle of the emoji
        parts.retain(|&part| !part.contains("fill=\"#FFCC4D\""));

        let stripped_emoji = parts.join(">");

        // svgload_buffer_with_opts is currently segfaulting
        // but svgload, svgload_with_opts and svgload_buffer do work 
        let emoji = ops::svgload_buffer_with_opts(stripped_emoji.as_bytes(), &ops::SvgloadBufferOptions {
            dpi: 72.0,
            scale: 1.0,
            unlimited: false,
            flags: ops::ForeignFlags::None,
            memory: false,
            access: ops::Access::Random,
            fail_on: ops::FailOn::None,
        })?;

        let ferris_length = ferris.get_width();
        let emoji_length = emoji.get_width(); // this is wrong, should be 36?
        let emoji_length = 36;

        //36 is the svg's normal length

        println!("{}", ferris_length);
        
        let ferris_clone = VipsImage::image_copy_memory(ferris.clone())?;

        /*
            Add is not perfect here.
            I can try a different operation.
            Or I could use the emoji as a mask to first delete certain pixels from ferris.
            Or I could except this bug and call it a feature as it does result in nice images.
        */

        ops::draw_image_with_opts(&ferris_clone, &emoji, (500/2)-(SCALEFACTOR as i32*36 as i32/2), 65, &ops::DrawImageOptions{
            mode: ops::CombineMode::Add
        })?;

        let ferris_clone_resized = ops::resize(&ferris_clone, 128.0/500.0)?;

        println!("{}", emoji_path.file_name().unwrap().to_owned().to_string_lossy().replace("svg", "png"));

        //ferris_clone_resized.image_write_to_file("x.png");

        //ferris_clone.image_set_kill(true);
        //ferris_clone1.image_set_kill(true);

        ferris_clone.image_write_to_file(format!("out/{}",emoji_path.file_name().unwrap().to_owned().to_string_lossy().replace("svg", "png")).as_str())?;
    }

    Ok(())
}
