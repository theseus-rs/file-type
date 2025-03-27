use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28758002: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_002,
        source_type: SourceType::Wikidata,
        name: "Inno Setup self-extracting archive",
        extensions: &["exe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
