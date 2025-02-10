use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_65595754: FileType = FileType {
    file_format: &FileFormat {
        id: 65_595_754,
        source_type: SourceType::Wikidata,
        name: "Variant Call Format",
        extensions: &["vcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
