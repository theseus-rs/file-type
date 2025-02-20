use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47196554: FileType = FileType {
    file_format: &FileFormat {
        id: 47_196_554,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Painting file format, version 5",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
