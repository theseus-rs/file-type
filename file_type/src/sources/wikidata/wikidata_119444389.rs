use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119444389: FileType = FileType {
    file_format: &FileFormat {
        id: 119_444_389,
        source_type: SourceType::Wikidata,
        name: "Comic Book Template",
        extensions: &["cbtx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
