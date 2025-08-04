use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub struct Section {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    avg: Option<Rgba<u8>>,
    sub_sections: Option<Vec<Section>>,
}

impl Section {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Section {
            x,
            y,
            w,
            h,
            avg: None,
            sub_sections: None,
        }
    }

    pub fn subdivide(&mut self, level: u32) {
        if level == 0 {
            return;
        };

        if self.w <= 10 || self.h <= 10 {
            return;
        }

        let mut sections = Vec::new();

        let w1 = self.w / 2;
        let h1 = self.h / 2;

        let w2 = self.w - w1;
        let h2 = self.h - h1;

        sections.push(Section::new(self.x, self.y, w1, h1));
        sections.push(Section::new(self.x + w1, self.y, w2, h1));
        sections.push(Section::new(self.x, self.y + h1, w1, h2));
        sections.push(Section::new(self.x + w1, self.y + h1, w2, h2));

        sections.iter_mut().for_each(|s| s.subdivide(level - 1));

        self.sub_sections = Some(sections);
    }

    pub fn fill_leaves(&self, mut img: DynamicImage) -> DynamicImage {
        if let Some(children) = self.sub_sections.as_ref() {
            for child in children {
                img = child.fill_leaves(img);
            }
        } else {
            img = self.fill(img);
        }

        return img;
    }

    pub fn fill(&self, mut img: DynamicImage) -> DynamicImage {
        let clr = self.avg.expect("You forgot to calculate avg first");

        for i in self.x..(self.x + self.w) {
            for j in self.y..(self.y + self.h) {
                img.put_pixel(i, j, clr);
            }
        }

        return img;
    }

    pub fn init(&mut self, img: &DynamicImage) {
        self.get_avg(img);
    }

    fn get_avg(&mut self, img: &DynamicImage) -> Rgba<u8> {
        if let Some(avg) = self.avg {
            return avg;
        } else {
            return self.calculate_avg(img);
        }
    }

    fn calculate_avg(&mut self, img: &DynamicImage) -> Rgba<u8> {
        let mut sum = [0u32, 0, 0, 0];
        let count;

        if let Some(sections) = self.sub_sections.as_deref_mut() {
            count = sections.len() as u32;

            for section in sections {
                acc(&mut sum, &section.get_avg(img).0);
            }
        } else {
            count = self.w * self.h;

            for i in self.x..(self.x + self.w) {
                for j in self.y..(self.y + self.h) {
                    let col = img.get_pixel(i, j);
                    acc(&mut sum, &col.0);
                }
            }
        }

        let avg = Rgba(sum.map(|s| (s / (count)) as u8));

        self.avg = Some(avg);
        return avg;
    }
}

fn acc(a: &mut [u32; 4], b: &[u8; 4]) {
    a[0] += b[0] as u32;
    a[1] += b[1] as u32;
    a[2] += b[2] as u32;
    a[3] += b[3] as u32;
}
