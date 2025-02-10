use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28009440: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_440,
        source_type: SourceType::Wikidata,
        name: "PCAPNG",
        extensions: &["pcapng"],
        media_types: &["application/vnd.tcpdump.pcap"],
        signatures: &[],
        related_formats: &[],
    },
};
