mod image_parser;

use crate::error::Result;
use crate::PaperMode;
use std::path::Path;
use x11rb::connection::Connection;
use x11rb::image::Image;
use x11rb::protocol::xproto::{
    AtomEnum, ChangeWindowAttributesAux, CloseDown, ConnectionExt, CreateGCAux, PropMode,
    Rectangle, Screen,
};
use x11rb::wrapper::ConnectionExt as _;

x11rb::atom_manager! {
    Atoms: AtomsCookie {
        xroot: b"_XROOTPMAP_ID",
        eroot: b"ESETROOT_PMAP_ID",
    }
}

fn set_atoms(conn: &impl Connection, screen: &Screen, pixmap: u32) -> Result<()> {
    let atoms = Atoms::new(conn)?.reply()?;
    conn.change_property32(
        PropMode::REPLACE,
        screen.root,
        atoms.xroot,
        AtomEnum::PIXMAP,
        &[pixmap],
    )?;
    conn.change_property32(
        PropMode::REPLACE,
        screen.root,
        atoms.eroot,
        AtomEnum::PIXMAP,
        &[pixmap],
    )?;
    conn.change_window_attributes(
        screen.root,
        &ChangeWindowAttributesAux::new().background_pixmap(pixmap),
    )?;
    Ok(())
}

// Create root pixmap and return the ids of pixmap and gc
fn create_root_pixmap(
    conn: &impl Connection,
    screen: &Screen,
    image: &Image,
) -> Result<(u32, u32)> {
    let pixmap = conn.generate_id()?;
    let gc = conn.generate_id()?;
    let rectangle = Rectangle {
        x: 0,
        y: 0,
        width: screen.width_in_pixels,
        height: screen.height_in_pixels,
    };

    conn.create_pixmap(
        screen.root_depth,
        pixmap,
        screen.root,
        screen.width_in_pixels,
        screen.height_in_pixels,
    )?;
    conn.create_gc(
        gc,
        pixmap,
        &CreateGCAux::new()
            .foreground(screen.black_pixel)
            .background(screen.white_pixel),
    )?;

    conn.poly_fill_rectangle(pixmap, gc, &[rectangle])?;
    image.put(conn, pixmap, gc, 0, 0)?;

    Ok((pixmap, gc))
}

pub fn set_wallpaper<P: AsRef<Path>>(wallpaper_path: P, mode: PaperMode) -> Result<()> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];
    let native_image = image_parser::parse_file(wallpaper_path, screen, mode)?;
    let (pixmap, gc) = create_root_pixmap(&conn, screen, &native_image)?;
    set_atoms(&conn, screen, pixmap)?;
    conn.set_close_down_mode(CloseDown::RETAIN_PERMANENT)?;
    conn.flush()?;
    conn.free_gc(gc)?;
    conn.free_pixmap(pixmap)?;
    Ok(())
}
