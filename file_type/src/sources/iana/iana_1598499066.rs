use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1598499066: FileType = FileType {
    file_format: &FileFormat {
        id: 1_598_499_066,
        source_type: SourceType::Iana,
        name: "vnd.dvb.esgcontainer",
        extensions: &[],
        media_types: &["application/vnd.dvb.esgcontainer"],
        signatures: &[],
        related_formats: &[],
    },
};
