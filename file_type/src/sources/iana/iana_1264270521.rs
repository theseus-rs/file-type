use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1264270521: FileType = FileType {
    file_format: &FileFormat {
        id: 1_264_270_521,
        source_type: SourceType::Iana,
        name: "vnd.iptc.g2.packageitem+xml",
        extensions: &[],
        media_types: &["application/vnd.iptc.g2.packageitem+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
