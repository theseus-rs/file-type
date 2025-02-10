use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121914796: FileType = FileType {
    file_format: &FileFormat {
        id: 121_914_796,
        source_type: SourceType::Wikidata,
        name: "Microsoft Agent File",
        extensions: &["acs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
