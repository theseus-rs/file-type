use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66310986: FileType = FileType {
    file_format: &FileFormat {
        id: 66_310_986,
        source_type: SourceType::Wikidata,
        name: "Shopping List file format",
        extensions: &["sl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
