use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_401: FileFormat = FileFormat {
    id: 401,
    source_type: SourceType::Linguist,
    name: "XProc",
    extensions: &["xpl", "xproc"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
