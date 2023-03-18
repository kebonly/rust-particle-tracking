// use std::rand;

fn main() {
    let my_optics: Optics = Optics {
        objective: NikonObjective::Blue,
        wavelength: 500e-9,
    };

    let my_detector: Detector = Detector { 
        pixel_size: 6.9e-6, 
        sensor_size: (544, 728), 
        read_noise: 15.0, 
        dark_current: 350.0, 
    };

    println!("This is my optical system: {:#?}", my_optics);
    println!("Resolution of optics: {}", my_optics.calculate_resolution());
    println!("Ideal pixel size: {}", my_optics.calculate_ideal_pixel_size());


    let positions: [[f32; 2]; 10] = [[10.0, 10.0]; 10];
    let sizes: [f32; 10] = [my_detector.pixel_size * 3.0; 10];
    let incident_photons: [u32; 10] = [1_000_000_000; 10];
    
    let my_particle_table: ParticleTable = ParticleTable { 
        positions: positions, 
        sizes: sizes, 
        incident_photons: incident_photons 
    };

    println!("This is my particle table: {:#?}", my_particle_table);
}

#[derive(Debug)]
pub enum NikonObjective {
    Yellow,
    Blue,
    White
}

/// Represents the optics of a microscope to determine resolution,
/// magnification, detector FOV, and detector pixel size
#[derive(Debug)]
struct Optics {
    objective: NikonObjective,
    wavelength: f32,
}

#[derive(Debug)]
struct Detector {
    pixel_size: f32,
    sensor_size: (u16, u16),
    read_noise: f32,
    dark_current: f32,
}

impl Detector {
    fn calculate_dark_current_noise(&self, exposure: f32) -> f32 {
        return &self.dark_current * exposure;
    }
}

impl Optics {
    fn calculate_resolution(&self) -> f32 {
        // Returns the Rayleigh resolution of the optical system
        let numerical_aperture: f32 = self.get_numerical_aperture();   
        return 0.61 * self.wavelength / numerical_aperture; 
    }

    fn get_mangification(&self) -> f32 {
        match &self.objective {
            NikonObjective::Yellow => 10.0,
            NikonObjective::Blue => 40.0,
            NikonObjective::White => 100.0,
        }
    }

    fn get_numerical_aperture(&self) -> f32 {
        match &self.objective {
            NikonObjective::Yellow => 0.25,
            NikonObjective::Blue => 0.17,
            NikonObjective::White => 0.17,
        }
    }

    fn calculate_ideal_pixel_size(&self) -> f32 {
        let magnification: f32 = self.get_mangification();
        return self.calculate_resolution() * magnification / 2.0;
    }
}

#[derive(Debug)]
struct ParticleTable {
    positions: [[f32; 2]; 10], // This should be a 2D array (# rows = # particles) (row,column position specification)
    sizes: [f32; 10], // vector of f64 units of pixels
    incident_photons: [u32; 10], // vector of u64, since photons are discrete
}

// fn generate_particle_table(size_image: (u16, u16)) -> ParticleTable {
//     let mut positions: [[f64, u16], u16] = [[0, size_image], size_image];


// }



// fn plot_all_particles() {
//     /// Plots particles onto 2D array using plot_one_particle() multiple times
//     /// 
//     /// # Arguments
//     /// 
// }

// fn plot_one_particle() {
//     /// A particle is defined by cosine function 
//     /// 
    
    
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn optics_calculate_resolution() {
        let optics: Optics = Optics {
            objective: NikonObjective::Blue,
            wavelength: 500e-9,
        };
        assert_eq!(0.000_001_794_117_647, optics.calculate_resolution());
    }

    #[test]
    fn optics_calculate_ideal_pixel_size() {
        let optics: Optics = Optics {
            objective: NikonObjective::Blue,
            wavelength: 500e-9,
        };
        assert_eq!(0.000_035_882_352_94, optics.calculate_ideal_pixel_size());
    }

    #[test]
    fn detector_calculate_dark_current_noise() {
        let detector: Detector = Detector { 
            pixel_size: 6.9e-6, 
            sensor_size: (544, 728), 
            read_noise: 15.0, 
            dark_current: 350.0, 
        };
        assert_eq!(0.035, detector.calculate_dark_current_noise(100e-6));
    }
}