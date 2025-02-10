use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3342397200: FileType = FileType {
    file_format: &FileFormat {
        id: 3_342_397_200,
        source_type: SourceType::Iana,
        name: "vnd.net2phone.commcenter.command",
        extensions: &[],
        media_types: &["text/vnd.net2phone.commcenter.command"],
        signatures: &[],
        related_formats: &[],
    },
};
