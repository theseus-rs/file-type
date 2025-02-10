use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48810278: FileType = FileType {
    file_format: &FileFormat {
        id: 48_810_278,
        source_type: SourceType::Wikidata,
        name: "DesignCAD Drawing",
        extensions: &["dc", "dc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
