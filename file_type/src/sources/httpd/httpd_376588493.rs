use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_376588493: FileType = FileType {
    file_format: &FileFormat {
        id: 376_588_493,
        source_type: SourceType::Httpd,
        name: "rpki ghostbusters",
        extensions: &["gbr"],
        media_types: &["application/rpki-ghostbusters"],
        signatures: &[],
        related_formats: &[],
    },
};
