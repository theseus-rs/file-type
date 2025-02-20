use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121816127: FileType = FileType {
    file_format: &FileFormat {
        id: 121_816_127,
        source_type: SourceType::Wikidata,
        name: "vCard 3",
        extensions: &["vcard", "vcf"],
        media_types: &["text/vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
