use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_674377195: FileType = FileType {
    file_format: &FileFormat {
        id: 674_377_195,
        source_type: SourceType::Iana,
        name: "vnd.3gpp2.sms",
        extensions: &[],
        media_types: &["application/vnd.3gpp2.sms"],
        signatures: &[],
        related_formats: &[],
    },
};
