use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_105: FileFormat = FileFormat {
    id: 105,
    source_type: SourceType::Linguist,
    name: "F#",
    extensions: &["fs", "fsi", "fsx"],
    media_types: &["text/x-fsharp"],
    signatures: &[],
    related_formats: &[],
};
