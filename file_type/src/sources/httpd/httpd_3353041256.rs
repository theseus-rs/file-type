use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3353041256: FileType = FileType {
    file_format: &FileFormat {
        id: 3_353_041_256,
        source_type: SourceType::Httpd,
        name: "mobius mbk",
        extensions: &["mbk"],
        media_types: &["application/vnd.mobius.mbk"],
        signatures: &[],
        related_formats: &[],
    },
};
