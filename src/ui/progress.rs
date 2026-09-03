use crate::solver::stop::CancelCondition;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Administra la representación visual del progreso durante la resolución del JSP.
pub struct Progress {
    spinner: ProgressBar,
}

impl Progress {
    /// Crea e inicia la animación del spinner de resolución.
    pub fn start() -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} Solving... elapsed: {elapsed_precise}")
                .unwrap(),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));

        Self { spinner }
    }

    /// Evalúa la condición de parada y finaliza la animación según el resultado.
    pub fn finish(self, cancel: &CancelCondition) {
        if cancel.is_cancelled() {
            self.finish_interrupted();
        } else {
            self.finish_success();
        }
    }

    /// Finaliza la animación con el formato para una conclusión exitosa.
    fn finish_success(self) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} Done in: {elapsed_precise}")
                .unwrap(),
        );
        self.spinner.finish();
    }

    /// Finaliza la animación con el formato para un proceso interrumpido.
    fn finish_interrupted(self) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.yellow} Interrupted after: {elapsed_precise}")
                .unwrap(),
        );
        self.spinner.finish();
    }
}
