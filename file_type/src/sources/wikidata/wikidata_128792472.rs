use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128792472: FileType = FileType {
    file_format: &FileFormat {
        id: 128_792_472,
        source_type: SourceType::Wikidata,
        name: "darcs patch format",
        extensions: &["darcspatch", "dpatch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
