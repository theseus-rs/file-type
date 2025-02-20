use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50223812: FileType = FileType {
    file_format: &FileFormat {
        id: 50_223_812,
        source_type: SourceType::Wikidata,
        name: "Bluetooth Snoop Packet Capture",
        extensions: &["log"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
