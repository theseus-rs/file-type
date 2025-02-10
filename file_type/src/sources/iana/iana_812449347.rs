use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_812449347: FileType = FileType {
    file_format: &FileFormat {
        id: 812_449_347,
        source_type: SourceType::Iana,
        name: "vnd.yamaha.openscoreformat.osfpvg+xml",
        extensions: &[],
        media_types: &["application/vnd.yamaha.openscoreformat.osfpvg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
