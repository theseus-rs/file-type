use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1724508731: FileType = FileType {
    file_format: &FileFormat {
        id: 1_724_508_731,
        source_type: SourceType::Iana,
        name: "vnd.sun.j2me.app-descriptor",
        extensions: &[],
        media_types: &["text/vnd.sun.j2me.app-descriptor"],
        signatures: &[],
        related_formats: &[],
    },
};
