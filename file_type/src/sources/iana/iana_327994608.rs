use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_327994608: FileType = FileType {
    file_format: &FileFormat {
        id: 327_994_608,
        source_type: SourceType::Iana,
        name: "vnd.gist.mx",
        extensions: &[],
        media_types: &["text/vnd.gist.mx"],
        signatures: &[],
        related_formats: &[],
    },
};
