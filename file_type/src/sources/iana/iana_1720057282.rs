use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1720057282: FileType = FileType {
    file_format: &FileFormat {
        id: 1_720_057_282,
        source_type: SourceType::Iana,
        name: "vnd.gentics.grd+json",
        extensions: &[],
        media_types: &["application/vnd.gentics.grd+json"],
        signatures: &[],
        related_formats: &[],
    },
};
