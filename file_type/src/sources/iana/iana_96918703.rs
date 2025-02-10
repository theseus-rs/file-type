use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_96918703: FileType = FileType {
    file_format: &FileFormat {
        id: 96_918_703,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.iufp",
        extensions: &[],
        media_types: &["audio/vnd.3gpp.iufp"],
        signatures: &[],
        related_formats: &[],
    },
};
