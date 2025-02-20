use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47245444: FileType = FileType {
    file_format: &FileFormat {
        id: 47_245_444,
        source_type: SourceType::Wikidata,
        name: "Microsoft Network Monitor Packet Capture",
        extensions: &["cap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
