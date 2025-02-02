use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_304: FileFormat = FileFormat {
    id: 304,
    source_type: SourceType::Linguist,
    name: "Python traceback",
    extensions: &["pytb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
