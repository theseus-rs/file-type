use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205995: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_995,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Color Map",
        extensions: &["imc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
