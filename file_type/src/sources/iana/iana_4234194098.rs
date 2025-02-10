use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4234194098: FileType = FileType {
    file_format: &FileFormat {
        id: 4_234_194_098,
        source_type: SourceType::Iana,
        name: "vnd.emclient.accessrequest+xml",
        extensions: &[],
        media_types: &["application/vnd.emclient.accessrequest+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
