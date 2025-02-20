use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2207671: FileType = FileType {
    file_format: &FileFormat {
        id: 2_207_671,
        source_type: SourceType::Wikidata,
        name: "SQX",
        extensions: &["sqx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
