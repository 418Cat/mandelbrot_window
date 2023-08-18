pub mod mand 
{
    use std::sync::atomic::AtomicU32;

    use num_complex::ComplexFloat;

    use super::mand_colors;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator, ParallelExtend};

    pub fn get_mand_point(coords: [f64; 2], limit: Option<u32>) -> u32
    {
        let mut z = num_complex::Complex64::new(0. as f64, 0. as f64);
        let c = num_complex::Complex64::new(coords[0], coords[1]);

        let limit = limit.unwrap_or(100);

        let mut iter: u32 = 0;

        if c.abs() > 2. as f64 {return 0}

        while (iter < limit) && (z.abs() < 2.)
        {
            z = z*z + c;
            iter+=1;
        }

        iter
    }

    pub fn get_mand_buff_img(start_rect: [f64; 2], rect_size: [f64; 2], res: [u32; 2], limit: Option<u32>, color_fn: mand_colors::ColorFn) -> image::RgbImage
    {
        let [res_fact_x, res_fact_y]: [f64; 2] = [rect_size[0] / res[0] as f64, rect_size[1] / res[1] as f64];

        let mut img_buff: image::RgbImage = image::RgbImage::new(res[0], res[1]);

        let limit = limit.unwrap_or(100);

        let color_fn: fn(u32, u32) -> colorsys::Rgb = mand_colors::get_fn_from_enum(color_fn);

        let mut mand_data: Vec<Vec<u32>> = Vec::new();

        let count = AtomicU32::new(0);

        mand_data.par_extend
        (
            (0..res[0]).into_par_iter().map
            (
                |x| 
                {
                    let mut x_mand: Vec<u32> = Vec::new();

                    x_mand.par_extend
                    (
                        (0..res[1]).into_par_iter().map
                        (
                    |y|
                            {
                                get_mand_point([start_rect[0] + res_fact_x * x as f64, start_rect[1] + res_fact_y * y as f64], Some(limit))
                            }
                        )
                    );
                    let nb = count.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                print!("\r[{}>{}] {:.2}%. iter_nb:{}       ", "=".repeat((nb as f32 / res[0] as f32 * 50.) as usize), " ".repeat(49 - (nb as f32 / res[0] as f32 * 50.) as usize), nb as f32 / res[0] as f32 * 100., limit);
                    x_mand
                }
            )
        );

        for x in 0..res[0] as usize
        {
            for y in 0..res[1] as usize
            {
                let pix_color = color_fn(mand_data[x][y], limit);
                img_buff.put_pixel(x as u32, y as u32, image::Rgb([pix_color.red() as u8, pix_color.green() as u8, pix_color.blue() as u8]));
            }
            
        }

        img_buff
    }
}

pub mod mand_colors
{

    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
    pub enum ColorFn
    {
        Colors1,
        Colors2,
        Colors3,
        Colors4,
        Colors5,
        Colors6,
        Colors7,
        Colors8,
        Colors9
    }

    pub fn get_fn_from_enum(funct: ColorFn) -> fn(u32, u32) -> colorsys::Rgb
    {
        match funct
        {
            ColorFn::Colors1 => color_1,
            ColorFn::Colors2 => color_2,
            ColorFn::Colors3 => color_3,
            ColorFn::Colors4 => color_4,
            ColorFn::Colors5 => color_5,
            ColorFn::Colors6 => color_6,
            ColorFn::Colors7 => color_7,
            ColorFn::Colors8 => color_8,
            ColorFn::Colors9 => color_9
        }
    }

    fn color_1(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        let hsl_color = colorsys::Hsl::new(((iter as f64 / max_iter as f64) * 7.).powf(3.5)%360., 100., (iter as f64 / max_iter as f64) * 75., None);
        colorsys::Rgb::from(&hsl_color).into()
    }

    fn color_2(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        let hsl_color = colorsys::Hsl::new(((iter as f64 / max_iter as f64) * 1.1).powf(1.1)%360., 100., (iter as f64 / max_iter as f64) * 100., None);
        colorsys::Rgb::from(&hsl_color).into()
    }

    fn color_3(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        let hsl_color = colorsys::Hsl::new((180. + (iter as f64 / max_iter as f64) * 1.01)%360., 75., (iter as f64 / max_iter as f64) * 50., None);
        colorsys::Rgb::from(&hsl_color).into()
    }

    fn color_4(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        let hsl_color = colorsys::Hsl::new((90. + (iter as f64 / max_iter as f64) * 5.)%360., 75., (iter as f64 / max_iter as f64) * 50., None);
        colorsys::Rgb::from(&hsl_color).into()
    }

    fn color_5(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        let hsl_color = colorsys::Hsl::new((90. + (iter as f64 / max_iter as f64) * 5.).powf(7.)%360., 75., (1.- 1./(iter as f64 / max_iter as f64 +1.)) * 50., None);
        colorsys::Rgb::from(&hsl_color).into()
    }

    fn color_6(iter: u32, _: u32) -> colorsys::Rgb
    {
        const A: f64 = 0.30102999566;//    1. / (2. as f64).log(10.);
        const B: f64 = A/4.24264068712;//  A  / (3. * (2. as f64 ).sqrt());
        const C: f64 = A/8.03041883308;//  A  / (7. * (3. as f64).powf(1./8.));

        let (red, green, blue): (f64, f64, f64) = 
        (
            255. * (1. - (A * iter as f64).cos())/2.,
            255. * (1. - (B * iter as f64).cos())/2.,
            255. * (1. - (C * iter as f64).cos())/2.
        );

        colorsys::Rgb::new(red, green, blue, None)
    }

    fn color_7(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        if iter == max_iter
        {
            colorsys::Rgb::new(255., 255., 255., None)
        }
        else
        {
            colorsys::Rgb::new(0., 0., 0., None)
        }
    }

    fn color_8(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        let col_fact = 255. * iter as f64 / max_iter as f64;
        colorsys::Rgb::new(col_fact, col_fact, col_fact, None)
    }

    fn color_9(iter: u32, max_iter: u32) -> colorsys::Rgb
    {
        let col_fact = 255. * iter as f64 / max_iter as f64;
        colorsys::Rgb::new(col_fact - 64., 255. * (1. - (iter as f64 / 50.0).cos())/2., col_fact.sin() * 255., None)
    }
}