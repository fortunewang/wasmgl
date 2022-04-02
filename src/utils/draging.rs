#[derive(Debug)]
pub struct Tracking<T> {
    ctx: T,
    original_onmousemove: Option<Option<js_sys::Function>>,
    original_onmouseup: Option<Option<js_sys::Function>>,
}

impl<T> Tracking<T> {
    fn new(ctx: T) -> Self {
        let document = gloo::utils::document();
        let original_onmousemove = Some(document.onmousemove());
        let original_onmouseup = Some(document.onmouseup());
        Self {
            ctx: ctx,
            original_onmousemove,
            original_onmouseup,
        }
    }

    fn ctx_copied(&self) -> T
    where
        T: Copy,
    {
        self.ctx
    }
}

impl<T> Drop for Tracking<T> {
    fn drop(&mut self) {
        let document = gloo::utils::document();
        if let Some(original_onmousemove) = self.original_onmousemove.take() {
            document.set_onmousemove(original_onmousemove.as_ref());
        }
        if let Some(original_onmouseup) = self.original_onmouseup.take() {
            document.set_onmousemove(original_onmouseup.as_ref());
        }
    }
}

#[derive(Debug, Default)]
pub struct Draging {
    tracking: Option<Tracking<(i32, i32)>>,
}

impl Draging {
    pub fn is_tracking(&self) -> bool {
        self.tracking.is_some()
    }

    pub fn stop_tracking(&mut self) {
        self.tracking = None;
    }

    pub fn onmousedown(&mut self, x: i32, y: i32) -> bool {
        if self.tracking.is_none() {
            self.tracking = Some(Tracking::new((x, y)));
            true
        } else {
            false
        }
    }

    pub fn onmousemove(&mut self, x: i32, y: i32) -> Option<(i32, i32)> {
        if let Some(tracking) = self.tracking.as_ref() {
            let (start_x, start_y) = tracking.ctx_copied();
            Some((x - start_x, y - start_y))
        } else {
            None
        }
    }

    pub fn onmouseup(&mut self, x: i32, y: i32) -> Option<(i32, i32)> {
        if let Some(tracking) = self.tracking.take() {
            let (start_x, start_y) = tracking.ctx_copied();
            Some((x - start_x, y - start_y))
        } else {
            None
        }
    }
}
