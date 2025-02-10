use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
