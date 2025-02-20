use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2776365282: FileType = FileType {
    file_format: &FileFormat {
        id: 2_776_365_282,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.channel",
        extensions: &[],
        media_types: &["application/vnd.uplanet.channel"],
        signatures: &[],
        related_formats: &[],
    },
};
