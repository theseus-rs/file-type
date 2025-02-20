use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7915770: FileType = FileType {
    file_format: &FileFormat {
        id: 7_915_770,
        source_type: SourceType::Wikidata,
        name: "Variant Call Format",
        extensions: &["vcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
