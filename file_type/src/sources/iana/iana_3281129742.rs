use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3281129742: FileType = FileType {
    file_format: &FileFormat {
        id: 3_281_129_742,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvueprofile+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvueprofile+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
