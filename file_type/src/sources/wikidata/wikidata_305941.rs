use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_305941: FileType = FileType {
    file_format: &FileFormat {
        id: 305_941,
        source_type: SourceType::Wikidata,
        name: "vCard",
        extensions: &["vcard", "vcf"],
        media_types: &["text/vcard"],
        signatures: &[],
        related_formats: &[],
    },
};
