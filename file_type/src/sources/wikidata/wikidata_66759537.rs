use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66759537: FileType = FileType {
    file_format: &FileFormat {
        id: 66_759_537,
        source_type: SourceType::Wikidata,
        name: "Excel Template",
        extensions: &["xltx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
