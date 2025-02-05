use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_818804755: FileFormat = FileFormat {
    id: 818_804_755,
    source_type: SourceType::Linguist,
    name: "Kaitai Struct",
    extensions: &["ksy"],
    media_types: &["text/x-yaml"],
    signatures: &[],
    related_formats: &[],
};
