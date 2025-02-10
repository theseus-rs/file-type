use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2218: FileType = FileType {
    file_format: &FileFormat {
        id: 2_218,
        source_type: SourceType::Pronom,
        name: "Ichitaro Document",
        extensions: &["jtd", "jtt", "$td"],
        media_types: &["application/x-js-taro"],
        signatures: &[],
        related_formats: &[],
    },
};
