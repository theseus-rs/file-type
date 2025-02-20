use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110040332: FileType = FileType {
    file_format: &FileFormat {
        id: 110_040_332,
        source_type: SourceType::Wikidata,
        name: "Harvard Graphics Presentation, version 1-3 PRS",
        extensions: &["prs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
