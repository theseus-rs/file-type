use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133082989: FileType = FileType {
    file_format: &FileFormat {
        id: 133_082_989,
        source_type: SourceType::Wikidata,
        name: "CD Architect Project File 4",
        extensions: &["cdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
