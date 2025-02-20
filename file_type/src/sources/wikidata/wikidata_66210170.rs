use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66210170: FileType = FileType {
    file_format: &FileFormat {
        id: 66_210_170,
        source_type: SourceType::Wikidata,
        name: "FrameMaker Book file format",
        extensions: &["book"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
