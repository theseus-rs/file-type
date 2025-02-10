use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117857305: FileType = FileType {
    file_format: &FileFormat {
        id: 117_857_305,
        source_type: SourceType::Wikidata,
        name: "Ricoh DX-1 Adapter/Fax Card file",
        extensions: &["ric"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
