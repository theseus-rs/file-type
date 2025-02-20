use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_65532981: FileType = FileType {
    file_format: &FileFormat {
        id: 65_532_981,
        source_type: SourceType::Wikidata,
        name: "Cookbook file format",
        extensions: &["mc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
