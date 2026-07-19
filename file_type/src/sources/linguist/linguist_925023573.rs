use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_925023573: FileType = FileType {
    file_format: &FileFormat {
        id: 925_023_573,
        source_type: SourceType::Linguist,
        name: "pkg-config",
        extensions: &["pc", "pc.in"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
