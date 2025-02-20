use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3655629976: FileType = FileType {
    file_format: &FileFormat {
        id: 3_655_629_976,
        source_type: SourceType::Httpd,
        name: "triscape mxs",
        extensions: &["mxs"],
        media_types: &["application/vnd.triscape.mxs"],
        signatures: &[],
        related_formats: &[],
    },
};
