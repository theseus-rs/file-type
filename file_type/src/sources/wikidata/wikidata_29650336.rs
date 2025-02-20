use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650336: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_336,
        source_type: SourceType::Wikidata,
        name: "Personal Information Exchange",
        extensions: &["p12", "pfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
