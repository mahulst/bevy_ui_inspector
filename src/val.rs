use bevy::ui::Val;

pub trait ValExt {
    fn px(self) -> Val;
    fn vh(self) -> Val;
    fn vw(self) -> Val;
    fn pct(self) -> Val;
}

impl ValExt for f32 {
    fn px(self) -> Val {
        Val::Px(self)
    }
    fn pct(self) -> Val {
        Val::Percent(self)
    }

    fn vh(self) -> Val {
        Val::Vh(self)
    }

    fn vw(self) -> Val {
        Val::Vw(self)
    }
}

