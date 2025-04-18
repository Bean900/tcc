use iced::{
    widget::{
        svg::{self, Handle},
        Svg,
    },
    Color, Theme,
};

use once_cell::sync::Lazy;

pub static IMAGE_COLLECTION: Lazy<ImageCollection> = Lazy::new(ImageCollection::new);

pub struct ImageCollection {
    // Progress images
    pub upload: SvgProgressImage,
    pub add_course: SvgProgressImage,
    pub calc: SvgProgressImage,
    pub result: SvgProgressImage,
    pub line: SvgProgressImage,
    pub next_line: SvgProgressImage,

    // Contact images
    pub pin: SvgImage,
    pub team: SvgImage,
    pub user_card: SvgImage,
}
pub struct SvgProgressImage {
    selected: Handle,
    next: Handle,
    previous: Handle,
}

pub struct SvgImage {
    img: Handle,
}

impl ImageCollection {
    fn new() -> Self {
        let upload = SvgProgressImage::new("/resources/upload.svg".to_string());
        let add_course = SvgProgressImage::new("/resources/add-course.svg".to_string());
        let calc = SvgProgressImage::new("/resources/calc.svg".to_string());
        let result = SvgProgressImage::new("/resources/result.svg".to_string());
        let line = SvgProgressImage::new("/resources/line.svg".to_string());
        let next_line = SvgProgressImage::new("/resources/next-line.svg".to_string());
        let pin = SvgImage::new("/resources/pin.svg".to_string());
        let team = SvgImage::new("/resources/team.svg".to_string());
        let user_card = SvgImage::new("/resources/user-card.svg".to_string());
        Self {
            upload,
            add_course,
            calc,
            result,
            line,
            next_line,
            pin,
            team,
            user_card,
        }
    }
}

impl SvgProgressImage {
    fn new(path: String) -> Self {
        Self {
            selected: load_svg(path.clone()),
            previous: load_svg(path.clone()),
            next: load_svg(path.clone()),
        }
    }

    fn get_style_previous(_: &Theme, status: svg::Status) -> svg::Style {
        let completed_step = Color::from_rgb(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0); // Forest Green
        let hover_step = Color::from_rgb(50.0 / 255.0, 205.0 / 255.0, 50.0 / 255.0); // Lime Green
        svg::Style {
            color: if status == svg::Status::Hovered {
                Some(hover_step)
            } else {
                Some(completed_step)
            },
        }
    }

    fn get_style_selected(_: &Theme, _: svg::Status) -> svg::Style {
        let current_step = Color::from_rgb(70.0 / 255.0, 130.0 / 255.0, 180.0 / 255.0); // Steel Blue
        svg::Style {
            color: Some(current_step),
        }
    }

    fn get_style_next(_: &Theme, _: svg::Status) -> svg::Style {
        let next_step = Color::from_rgb(169.0 / 255.0, 169.0 / 255.0, 169.0 / 255.0); // Dark Gray
        svg::Style {
            color: Some(next_step),
        }
    }

    pub fn get_selected(&self) -> Svg {
        Svg::new(self.selected.clone()).style(SvgProgressImage::get_style_selected)
    }

    pub fn get_next(&self) -> Svg {
        Svg::new(self.next.clone()).style(SvgProgressImage::get_style_next)
    }

    pub fn get_previous(&self) -> Svg {
        Svg::new(self.previous.clone()).style(SvgProgressImage::get_style_previous)
    }
}

impl SvgImage {
    fn new(path: String) -> Self {
        Self {
            img: load_svg(path.clone()),
        }
    }

    pub fn get(&self, size: u16) -> Svg {
        Svg::new(self.img.clone()).height(size).width(size) //.style(|_, _| svg::Style {            color: Some(Color::from_rgb(243.0 / 255.0, 156.0 / 255.0, 18.0 / 255.0)),     })
    }
}

fn load_svg(path: String) -> Handle {
    Handle::from_path(format!("{}{}", env!("CARGO_MANIFEST_DIR"), path))
}
