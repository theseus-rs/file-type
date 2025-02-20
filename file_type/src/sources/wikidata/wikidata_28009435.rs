use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28009435: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_435,
        source_type: SourceType::Wikidata,
        name: "PCAP",
        extensions: &["pcap"],
        media_types: &["application/vnd.tcpdump.pcap"],
        signatures: &[],
        related_formats: &[],
    },
};
