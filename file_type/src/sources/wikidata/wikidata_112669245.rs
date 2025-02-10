use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112669245: FileType = FileType {
    file_format: &FileFormat {
        id: 112_669_245,
        source_type: SourceType::Wikidata,
        name: "Lightscape Layers",
        extensions: &["lay"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
