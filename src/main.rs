extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};

struct ImageViewer {
    asset: Asset<Image>,
    lenna: Asset<Image>,
}

impl State for ImageViewer {
    fn new() -> Result<ImageViewer> {
        let asset = Asset::new(Image::load("image.png"));
        let lenna = Asset::new(Image::load("lenna.png"));
        Ok(ImageViewer { asset, lenna })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.asset.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })?;
        self.lenna.execute(|image| {
            window.draw(&image.area().with_center((400, 500)), Img(&image));
            Ok(())
        })?;
        Ok(())
    }
}

fn main() {
    run::<ImageViewer>("Image Example", Vector::new(800, 600), Settings {
        icon_path: Some("image.png"),
        ..Settings::default()
    });
}
