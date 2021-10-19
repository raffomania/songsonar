#[macro_export]
macro_rules! get_all_cursor_pages {
    ($after: ident, $get: block) => {{
        let $after: Option<&str> = None;
        let mut page: aspotify::CursorPage<_> = $get;
        let mut results: Vec<_> = page.items;
        while let Some(ref $after) = page.cursors.after {
            let $after = Some($after.as_str());
            page = $get;
            results.append(&mut page.items);
        }
        results
    }};
}

#[macro_export]
macro_rules! get_all_pages {
    ($offset: ident, $get: block) => {{
        let $offset: usize = 0;
        let mut page: aspotify::Page<_> = $get;
        let mut results: Vec<_> = page.items;
        while page.offset + page.limit < page.total {
            let $offset = page.offset + page.limit;
            page = $get;
            results.append(&mut page.items);
        }
        results
    }};
}
