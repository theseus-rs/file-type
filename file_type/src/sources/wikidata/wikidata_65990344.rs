use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_65990344: FileType = FileType {
    file_format: &FileFormat {
        id: 65_990_344,
        source_type: SourceType::Wikidata,
        name: "Adobe Premiere Project",
        extensions: &["ppj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
