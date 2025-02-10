use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1931585: FileType = FileType {
    file_format: &FileFormat {
        id: 1_931_585,
        source_type: SourceType::Wikidata,
        name: "Microsoft Digital Video Recording",
        extensions: &["dvr-ms"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
