use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47538998: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_998,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Landscape Library",
        extensions: &["lli"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
