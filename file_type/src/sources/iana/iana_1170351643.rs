use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1170351643: FileType = FileType {
    file_format: &FileFormat {
        id: 1_170_351_643,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.listcmd",
        extensions: &[],
        media_types: &["application/vnd.uplanet.listcmd"],
        signatures: &[],
        related_formats: &[],
    },
};
