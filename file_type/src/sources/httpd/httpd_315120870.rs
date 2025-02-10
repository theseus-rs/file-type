use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_315120870: FileType = FileType {
    file_format: &FileFormat {
        id: 315_120_870,
        source_type: SourceType::Httpd,
        name: "geogebra tool",
        extensions: &["ggt"],
        media_types: &["application/vnd.geogebra.tool"],
        signatures: &[],
        related_formats: &[],
    },
};
