use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126822716: FileType = FileType {
    file_format: &FileFormat {
        id: 126_822_716,
        source_type: SourceType::Wikidata,
        name: "Visual F# Source File",
        extensions: &["fs"],
        media_types: &["text/x-fsharp"],
        signatures: &[],
        related_formats: &[],
    },
};
