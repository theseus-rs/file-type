use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66689208: FileType = FileType {
    file_format: &FileFormat {
        id: 66_689_208,
        source_type: SourceType::Wikidata,
        name: "Access Database (Pocket Access for Windows CE)",
        extensions: &["cdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
