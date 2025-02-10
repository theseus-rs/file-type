use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2873474993: FileType = FileType {
    file_format: &FileFormat {
        id: 2_873_474_993,
        source_type: SourceType::Iana,
        name: "vnd.wfa.p2p",
        extensions: &[],
        media_types: &["application/vnd.wfa.p2p"],
        signatures: &[],
        related_formats: &[],
    },
};
