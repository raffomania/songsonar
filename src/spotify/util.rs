use aspotify::{Client, CursorPage};

use crate::basics::*;

pub async fn fetch_all_cursors<T>(
    client: Client,
    page: CursorPage<T>,
) -> Result<Vec<T>> {
    let mut results = page.items;
    if let Some(after) = page.cursors.after {}
    Ok(results)
}
