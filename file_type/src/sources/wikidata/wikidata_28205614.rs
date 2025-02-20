use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205614: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_614,
        source_type: SourceType::Wikidata,
        name: "RIPscrip version 2 Icon Mask",
        extensions: &["bmm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
