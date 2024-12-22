use std::collections::HashSet;

lazy_static::lazy_static! {
    pub static ref CONTENT_MIME_TYPES: HashSet<&'static str> = {
        HashSet::<&'static str>::from_iter(vec![
            "application/epub+zip",
            "application/font-woff",
            "application/font-woff2",
            "application/gzip",
            "application/javascript",
            "application/json",
            "application/mathml+xml",
            "application/oebps-package+xml",
            "application/ogg",
            "application/pdf",
            "application/pls+xml",
            "application/smil+xml",
            "application/vnd.adobe.xpgt",
            "application/vnd.ms-fontobject",
            "application/vnd.ms-opentype",
            "application/x-7z-compressed",
            "application/x-dtbncx+xml",
            "application/x-dtbook+xml",
            "application/x-font-otf",
            "application/x-font-truetype",
            "application/x-font-ttf",
            "application/x-rar-compressed",
            "application/x-tar",
            "application/x-yaml",
            "application/xenc+xml",
            "application/xhtml+xml",
            "application/xinclude+xml",
            "application/xlink+xml",
            "application/xml",
            "application/xml-dtd",
            "application/xpath+xml",
            "application/xpointer+xml",
            "application/xproc+xml",
            "application/xquery+xml",
            "application/xsig+xml",
            "application/xslt+xml",
            "application/zip",
            "audio/aac",
            "audio/flac",
            "audio/mp4",
            "audio/mpeg",
            "audio/ogg",
            "audio/wav",
            "audio/x-ms-wma",
            "audio/x-wav",
            "font/otf",
            "font/ttf",
            "font/woff",
            "font/woff2",
            "image/bmp",
            "image/gif",
            "image/jpeg",
            "image/png",
            "image/svg+xml",
            "image/tiff",
            "image/webp",
            "image/x-icon",
            "text/css",
            "text/csv",
            "text/html",
            "text/javascript",
            "text/markdown",
            "text/plain",
            "text/xml",
            "video/mp4",
            "video/mpeg",
            "video/ogg",
            "video/quicktime",
            "video/webm",
            "video/x-ms-wmv",
            "video/x-msvideo",
        ])
    };
    pub static ref HTML_MIME_TYPES: HashSet<&'static str> = {
        HashSet::<&'static str>::from_iter(vec![
            "application/xhtml+xml",
            "text/html",
        ])
    };
    pub static ref MARKDOWN_MIME_TYPES: HashSet<&'static str> = {
        HashSet::<&'static str>::from_iter(vec![
            "text/markdown",
        ])
    };
}

pub const STANDARD_MARKDOWN_MIME_TYPE: &'static str = "text/markdown";

pub fn is_content_mime_type(mime_type: impl AsRef<str>) -> bool {
    CONTENT_MIME_TYPES.contains(mime_type.as_ref())
}

pub fn is_html_mime_type(mime_type: impl AsRef<str>) -> bool {
    HTML_MIME_TYPES.contains(mime_type.as_ref())
}

pub fn is_markdown_mime_type(mime_type: impl AsRef<str>) -> bool {
    MARKDOWN_MIME_TYPES.contains(mime_type.as_ref())
}
