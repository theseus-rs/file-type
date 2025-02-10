use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_110: FileType = FileType {
    file_format: &FileFormat {
        id: 110,
        source_type: SourceType::Pronom,
        name: "AutoLISP Menu Source File",
        extensions: &["mnl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
