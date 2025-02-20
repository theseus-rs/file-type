use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122305680: FileType = FileType {
    file_format: &FileFormat {
        id: 122_305_680,
        source_type: SourceType::Wikidata,
        name: "PressRelease Project",
        extensions: &["prp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
