use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
