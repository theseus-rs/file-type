use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1727175336: FileType = FileType {
    file_format: &FileFormat {
        id: 1_727_175_336,
        source_type: SourceType::Iana,
        name: "vnd.dts.hd",
        extensions: &[],
        media_types: &["audio/vnd.dts.hd"],
        signatures: &[],
        related_formats: &[],
    },
};
