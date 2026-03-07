mod music;
mod sfx;

pub use self::{music::*, sfx::*};

use crate::prelude::*;

#[derive(Clone)]
pub struct Context {
    pub geng: Geng,
    pub assets: Rc<Assets>,
    pub music: Rc<MusicManager>,
    pub sfx: Rc<SfxManager>,
    options: Rc<RefCell<Options>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Options {
    pub master_volume: f32,
    pub music_volume: f32,
}

impl Options {
    pub fn volume_sfx(&self) -> f32 {
        self.master_volume
    }

    pub fn volume_music(&self) -> f32 {
        self.master_volume * self.music_volume
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            music_volume: 1.0,
        }
    }
}

impl Context {
    pub async fn new(geng: &Geng, assets: &Rc<Assets>) -> Result<Self> {
        let options: Options = preferences::load(crate::OPTIONS_STORAGE).unwrap_or_default();
        let options_rc = Rc::new(RefCell::new(Options::default()));
        let ctx = Self {
            geng: geng.clone(),
            assets: assets.clone(),
            music: Rc::new(MusicManager::new(geng.clone())),
            sfx: Rc::new(SfxManager::new(geng.clone(), options_rc.clone())),
            options: options_rc,
        };
        ctx.force_set_options(options);
        Ok(ctx)
    }

    pub fn get_options(&self) -> Options {
        self.options.borrow().clone()
    }

    pub fn set_options(&self, options: Options) {
        let old = self.options.borrow();
        if *old != options {
            drop(old);
            self.force_set_options(options);
        }
    }

    fn force_set_options(&self, options: Options) {
        let mut old = self.options.borrow_mut();

        self.music
            .set_volume(options.master_volume * options.music_volume);

        preferences::save(crate::OPTIONS_STORAGE, &options);
        *old = options;
    }
}
