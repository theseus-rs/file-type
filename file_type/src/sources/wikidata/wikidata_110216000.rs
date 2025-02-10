use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110216000: FileType = FileType {
    file_format: &FileFormat {
        id: 110_216_000,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication, version 1-3",
        extensions: &["ppp", "ppt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
