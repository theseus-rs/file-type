use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3932157629: FileType = FileType {
    file_format: &FileFormat {
        id: 3_932_157_629,
        source_type: SourceType::Httpd,
        name: "font tdpfr",
        extensions: &["pfr"],
        media_types: &["application/font-tdpfr"],
        signatures: &[],
        related_formats: &[],
    },
};
