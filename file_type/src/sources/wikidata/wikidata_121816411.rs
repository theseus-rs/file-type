use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121816411: FileType = FileType {
    file_format: &FileFormat {
        id: 121_816_411,
        source_type: SourceType::Wikidata,
        name: "vCard 4",
        extensions: &["vcard", "vcf"],
        media_types: &["text/vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
