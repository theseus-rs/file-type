use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205979: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_979,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive I Color Channel",
        extensions: &["imi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
