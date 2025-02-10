use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51370033: FileType = FileType {
    file_format: &FileFormat {
        id: 51_370_033,
        source_type: SourceType::Wikidata,
        name: "Freelance file format",
        extensions: &["pre"],
        media_types: &["application/vnd.lotus-freelance"],
        signatures: &[],
        related_formats: &[],
    },
};
