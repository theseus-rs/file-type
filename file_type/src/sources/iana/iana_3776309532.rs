use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3776309532: FileType = FileType {
    file_format: &FileFormat {
        id: 3_776_309_532,
        source_type: SourceType::Iana,
        name: "vnd.cns.inf2",
        extensions: &[],
        media_types: &["image/vnd.cns.inf2"],
        signatures: &[],
        related_formats: &[],
    },
};
