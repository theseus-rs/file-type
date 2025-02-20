use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122228898: FileType = FileType {
    file_format: &FileFormat {
        id: 122_228_898,
        source_type: SourceType::Wikidata,
        name: "Oracle Password Hash",
        extensions: &["orc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
