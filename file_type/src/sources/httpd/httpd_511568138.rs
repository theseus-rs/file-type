use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_511568138: FileType = FileType {
    file_format: &FileFormat {
        id: 511_568_138,
        source_type: SourceType::Httpd,
        name: "geogebra file",
        extensions: &["ggb"],
        media_types: &["application/vnd.geogebra.file"],
        signatures: &[],
        related_formats: &[],
    },
};
