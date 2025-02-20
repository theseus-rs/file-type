use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167857: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_857,
        source_type: SourceType::Wikidata,
        name: "P-touch Editor Lite Label",
        extensions: &["lbt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
