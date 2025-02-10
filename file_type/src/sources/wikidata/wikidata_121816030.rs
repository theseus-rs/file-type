use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121816030: FileType = FileType {
    file_format: &FileFormat {
        id: 121_816_030,
        source_type: SourceType::Wikidata,
        name: "vCard 2.1",
        extensions: &["vcard", "vcf"],
        media_types: &["text/vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
