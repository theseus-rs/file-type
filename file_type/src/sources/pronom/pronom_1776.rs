use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1776: FileType = FileType {
    file_format: &FileFormat {
        id: 1_776,
        source_type: SourceType::Pronom,
        name: "Microsoft Windows Movie Maker File",
        extensions: &["mswmm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
