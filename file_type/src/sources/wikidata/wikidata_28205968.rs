use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205968: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_968,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Blue Channel",
        extensions: &["imb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
