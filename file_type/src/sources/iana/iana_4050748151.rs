use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4050748151: FileType = FileType {
    file_format: &FileFormat {
        id: 4_050_748_151,
        source_type: SourceType::Iana,
        name: "vnd.ims.lti.v2.toolsettings+json",
        extensions: &[],
        media_types: &["application/vnd.ims.lti.v2.toolsettings+json"],
        signatures: &[],
        related_formats: &[],
    },
};
