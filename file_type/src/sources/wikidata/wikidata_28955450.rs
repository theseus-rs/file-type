use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28955450: FileType = FileType {
    file_format: &FileFormat {
        id: 28_955_450,
        source_type: SourceType::Wikidata,
        name: "GenBank file format",
        extensions: &["gb", "gbk", "gen"],
        media_types: &["chemical/x-genbank"],
        signatures: &[],
        related_formats: &[],
    },
};
