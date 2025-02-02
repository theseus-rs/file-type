use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_950967261: FileFormat = FileFormat {
    id: 950_967_261,
    source_type: SourceType::Linguist,
    name: "Win32 Message File",
    extensions: &["mc"],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};
