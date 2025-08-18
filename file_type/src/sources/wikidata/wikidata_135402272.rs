use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135402272: FileType = FileType {
    file_format: &FileFormat {
        id: 135_402_272,
        source_type: SourceType::Wikidata,
        name: "Point Cloud Project file",
        extensions: &["rcp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
