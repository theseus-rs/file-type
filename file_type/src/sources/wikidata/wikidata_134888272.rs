use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134888272: FileType = FileType {
    file_format: &FileFormat {
        id: 134_888_272,
        source_type: SourceType::Wikidata,
        name: "Solid Edge XML design",
        extensions: &["cmp_xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
