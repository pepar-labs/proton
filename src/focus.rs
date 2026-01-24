
use crate::style::Rect;

pub type FocusId = u32;

#[derive(Debug, Clone)]
pub struct FocusableRect {
    pub id: FocusId,
    pub rect: Rect,
}

#[derive(Debug, Clone, Default)]
pub struct FocusState {
    pub focused_id: Option<FocusId>,
    focusables: Vec<FocusableRect>,
}

impl FocusState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, id: FocusId, rect: Rect) {
        self.focusables.retain(|f| f.id != id);
        self.focusables.push(FocusableRect { id, rect });
    }

    pub fn clear_focusables(&mut self) {
        self.focusables.clear();
    }

    pub fn focusables(&self) -> &[FocusableRect] {
        &self.focusables
    }

    pub fn focus_next(&mut self) {
        if self.focusables.is_empty() {
            self.focused_id = None;
            return;
        }

        self.focused_id = match self.focused_id {
            None => self.focusables.first().map(|f| f.id),
            Some(current) => {
                let current_idx = self.focusables.iter().position(|f| f.id == current);
                match current_idx {
                    Some(idx) => {
                        let next_idx = (idx + 1) % self.focusables.len();
                        Some(self.focusables[next_idx].id)
                    }
                    None => self.focusables.first().map(|f| f.id),
                }
            }
        };
    }

    pub fn focus_prev(&mut self) {
        if self.focusables.is_empty() {
            self.focused_id = None;
            return;
        }

        self.focused_id = match self.focused_id {
            None => self.focusables.last().map(|f| f.id),
            Some(current) => {
                let current_idx = self.focusables.iter().position(|f| f.id == current);
                match current_idx {
                    Some(idx) => {
                        let prev_idx = if idx == 0 {
                            self.focusables.len() - 1
                        } else {
                            idx - 1
                        };
                        Some(self.focusables[prev_idx].id)
                    }
                    None => self.focusables.last().map(|f| f.id),
                }
            }
        };
    }

    pub fn is_focused(&self, id: FocusId) -> bool {
        self.focused_id == Some(id)
    }

    pub fn set_focus(&mut self, id: FocusId) {
        self.focused_id = Some(id);
    }

    pub fn clear_focus(&mut self) {
        self.focused_id = None;
    }

    pub fn focus_first_if_none(&mut self) {
        if self.focused_id.is_none() && !self.focusables.is_empty() {
            self.focused_id = self.focusables.first().map(|f| f.id);
        }
    }

    pub fn focused_rect(&self) -> Option<&Rect> {
        self.focused_id.and_then(|id| {
            self.focusables.iter().find(|f| f.id == id).map(|f| &f.rect)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rect() -> Rect {
        Rect { x: 0.0, y: 0.0, width: 100.0, height: 50.0 }
    }

    #[test]
    fn test_focus_navigation() {
        let mut state = FocusState::new();
        state.register(1, make_rect());
        state.register(2, make_rect());
        state.register(3, make_rect());

        assert_eq!(state.focused_id, None);

        state.focus_next();
        assert_eq!(state.focused_id, Some(1));

        state.focus_next();
        assert_eq!(state.focused_id, Some(2));

        state.focus_next();
        assert_eq!(state.focused_id, Some(3));

        state.focus_next();
        assert_eq!(state.focused_id, Some(1));
    }

    #[test]
    fn test_focus_prev() {
        let mut state = FocusState::new();
        state.register(1, make_rect());
        state.register(2, make_rect());

        state.set_focus(2);
        state.focus_prev();
        assert_eq!(state.focused_id, Some(1));

        state.focus_prev();
        assert_eq!(state.focused_id, Some(2));
    }
}
