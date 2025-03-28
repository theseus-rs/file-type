use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130478829: FileType = FileType {
    file_format: &FileFormat {
        id: 130_478_829,
        source_type: SourceType::Wikidata,
        name: "pkg-config file format",
        extensions: &["pc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
