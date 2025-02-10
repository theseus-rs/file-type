use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_315120870: FileType = FileType {
    file_format: &FileFormat {
        id: 315_120_870,
        source_type: SourceType::Iana,
        name: "vnd.geogebra.tool",
        extensions: &[],
        media_types: &["application/vnd.geogebra.tool"],
        signatures: &[],
        related_formats: &[],
    },
};
