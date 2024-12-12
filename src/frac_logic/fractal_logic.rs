use std::sync::mpsc::Sender;

use crate::app::SlaveMessage;
use crate::frac_logic::{CanvasCoords, RenderSettings};
use crate::fractals::{Fractal, FractalClos, FRACTALS};
use crate::helpers::Vec2;

use rayon::prelude::*;
use rug::ops::CompleteRound;
use rug::{Complex, Float};

use super::gpu_util::SendSlaveMessage;

const INITIAL_CANVAS_WIDTH: i32 = 5;
pub(crate) type DivergMatrix = Vec<Vec<i32>>;

impl RenderSettings {
    /// Returns a divergence matrix of the specified size.
    fn _get_diverg_matrix_with_status(
        &self,
        size: &Vec2<i32>,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> DivergMatrix {
        // Get the canvas coordinates of each row
        let half_x = size.x / 2;
        let half_y = size.y / 2;
        let cell_size = self.get_plane_wid() / size.x;

        let div_matrix = (-half_y..=-half_y + size.y - 1)
            .into_par_iter()
            .map(|y| {
                let line = (-half_x..=-half_x + size.x)
                    .into_par_iter()
                    .map(|x| {
                        (self.get_frac_clos())(
                            self.coord_to_c_with_cell_size(CanvasCoords::new(x, y), &cell_size),
                            self,
                        )
                    })
                    .collect();

                // Send an update to the parent process,
                // indicating that one line has been rendered.
                sender.send(SlaveMessage::LineRender).unwrap();

                line
            })
            .collect();

        // Send a message to the parent process indicating that the screenshot finished,
        // and it should now wait for the result transfer through the `JoinHandle`.
        sender.send(SlaveMessage::JobFinished).unwrap();

        div_matrix
    }

    /// Returns a divergence matrix, and send an update to the channel
    /// after each line is rendered.
    pub(crate) fn get_diverg_matrix_with_status(
        &self,
        size: &Vec2<i32>,
        sender: &Sender<SlaveMessage>,
    ) -> DivergMatrix {
        self._get_diverg_matrix_with_status(size, Some(sender))
    }

    /// Returns a divergence matrix without sending any updates.
    pub(crate) fn get_diverg_matrix(&self, size: &Vec2<i32>) -> DivergMatrix {
        self._get_diverg_matrix_with_status(size, None)
    }

    /// Use the provided cell size to find the complex number
    /// corresponding to the given canvas position.
    /// Uses the configured precision and canvas position.
    pub(crate) fn coord_to_c_with_cell_size(
        &self,
        coords: CanvasCoords,
        cell_size: &Float,
    ) -> Complex {
        Complex::with_val(self.prec, (coords.x * cell_size, coords.y * cell_size)) + &self.pos
    }

    /// Takes coordinates on the canvas and return the
    /// complex number at the corresponding position on the complex plane.
    pub(crate) fn coord_to_c(&self, coords: CanvasCoords) -> Complex {
        self.coord_to_c_with_cell_size(coords, &self.cell_size)
    }

    /// Set the position of the canvas to the selected fractal's fixed default position.
    pub(crate) fn reset_pos(&mut self) {
        self.pos = Complex::with_val(self.prec, self.get_frac_obj().default_pos);
    }

    /// Returns the selected fractal's closure.
    pub(crate) fn get_frac_clos(&self) -> FractalClos {
        FRACTALS[self.frac_index].get
    }

    /// Returns the selected fractal.
    pub(crate) fn get_frac_obj(&self) -> &Fractal {
        &FRACTALS[self.frac_index]
    }

    /// Converts ratatui coordinates to canvas coordinates.
    pub(crate) fn ratatui_to_canvas_coords(&self, x: u16, y: u16) -> CanvasCoords {
        // I don't understand how this works
        CanvasCoords::new(
            x as i32 - 1 - self.canvas_size.x / 2,
            y as i32 * -2 + 1 + self.canvas_size.y / 2,
        )
    }

    /// Set the cell size so that the total width of the canvas is set to the specified size.
    pub(crate) fn set_width(&mut self, width: Float) {
        self.cell_size = width / self.canvas_size.x;
    }

    /// Set the cell size so that the total width of the canvas is 4 on the real axis.
    pub(crate) fn reset_cell_size(&mut self) {
        self.cell_size = self.get_default_cell_size();
    }

    /// Return the cell size so that the total width of the canvas is 4 on the real axis.
    pub(crate) fn get_default_cell_size(&self) -> Float {
        Float::with_val(self.prec, INITIAL_CANVAS_WIDTH) / self.canvas_size.x
    }

    /// Get the canvas width in the complex plane.
    pub(crate) fn get_plane_wid(&self) -> Float {
        (self.canvas_size.x * &self.cell_size).complete(self.prec)
    }
}
