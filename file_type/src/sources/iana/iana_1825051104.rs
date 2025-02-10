use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1825051104: FileType = FileType {
    file_format: &FileFormat {
        id: 1_825_051_104,
        source_type: SourceType::Iana,
        name: "vnd.bluetooth.ep.oob",
        extensions: &[],
        media_types: &["application/vnd.bluetooth.ep.oob"],
        signatures: &[],
        related_formats: &[],
    },
};
