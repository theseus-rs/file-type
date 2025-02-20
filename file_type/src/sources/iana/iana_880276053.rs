use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_880276053: FileType = FileType {
    file_format: &FileFormat {
        id: 880_276_053,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.tunnel-udpencap",
        extensions: &[],
        media_types: &["application/vnd.yamaha.tunnel-udpencap"],
        signatures: &[],
        related_formats: &[],
    },
};
