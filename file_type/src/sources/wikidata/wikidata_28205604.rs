use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205604: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_604,
        source_type: SourceType::Wikidata,
        name: "RIPscrip version 1 Icon Mask",
        extensions: &["msk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
