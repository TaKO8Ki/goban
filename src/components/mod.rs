pub mod command;
pub mod connections;
pub mod databases;
pub mod error;
pub mod record_table;
pub mod tab;
pub mod table;
pub mod table_filter;
pub mod table_status;
pub mod table_value;
pub mod utils;

pub use command::{CommandInfo, CommandText};
pub use connections::ConnectionsComponent;
pub use databases::DatabasesComponent;
pub use error::ErrorComponent;
pub use record_table::RecordTableComponent;
pub use tab::TabComponent;
pub use table::TableComponent;
pub use table_filter::TableFilterComponent;
pub use table_status::TableStatusComponent;
pub use table_value::TableValueComponent;

use anyhow::Result;
use async_trait::async_trait;
use tui::{backend::Backend, layout::Rect, Frame};

#[derive(Copy, Clone)]
pub enum ScrollType {
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
}

pub trait DrawableComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect, focused: bool) -> Result<()>;
}

/// base component trait
#[async_trait]
pub trait Component {
    fn event(&mut self, key: crate::event::Key) -> Result<()>;

    // async fn async_event(&mut self, key: crate::event::Key) -> Result<()>;

    fn focused(&self) -> bool {
        false
    }

    fn focus(&mut self, _focus: bool) {}

    fn is_visible(&self) -> bool {
        true
    }

    fn hide(&mut self) {}

    fn show(&mut self) -> Result<()> {
        Ok(())
    }

    fn toggle_visible(&mut self) -> Result<()> {
        if self.is_visible() {
            self.hide();
            Ok(())
        } else {
            self.show()
        }
    }
}
