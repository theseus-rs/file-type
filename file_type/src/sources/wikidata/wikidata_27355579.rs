use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27355579: FileType = FileType {
    file_format: &FileFormat {
        id: 27_355_579,
        source_type: SourceType::Wikidata,
        name: "ADRG Overview Image File",
        extensions: &["ovr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
