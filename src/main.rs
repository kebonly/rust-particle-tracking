fn main() {
    let my_optics: Optics = Optics {
        numerical_aperture: 0.25,
        wavelength: 500e-9,
        pixel_size: 6.9e-6,
        magnification: 25.0,
        fov: (544, 728)
    };

    println!("Resolution of optics: {}", my_optics.calculate_resolution());
    println!("Ideal pixel size: {}", my_optics.calculate_ideal_pixel_size());
}

/// Represents the optics of a microscope to determine resolution,
/// magnification, detector FOV, and detector pixel size
struct Optics {
    numerical_aperture: f64,
    wavelength: f64,
    pixel_size: f64,
    magnification: f64,
    fov: (u16, u16),
}

// struct ParticleTable {
//     positions: [[f64; 2]; 10], // This should be a 2D array (# rows = # particles) (row,column position specification)
//     sizes: [f64; 10], // vector of f64 units of pixels
//     incident_photons: [u64; 10], // vector of u64, since photons are discrete
// }

impl Optics {
    fn calculate_resolution(&self) -> f64 {
        // Returns the Rayleigh resolution of the optical system
        return 0.61 * self.wavelength / self.numerical_aperture;
    }

    fn calculate_ideal_pixel_size(&self) -> f64 {
        return self.calculate_resolution() * self.magnification / 2.0;
    }


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