use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113137926: FileType = FileType {
    file_format: &FileFormat {
        id: 113_137_926,
        source_type: SourceType::Wikidata,
        name: "Wireshark nanosecond libpcap",
        extensions: &["cap", "pcap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
