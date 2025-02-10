use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_70477993: FileType = FileType {
    file_format: &FileFormat {
        id: 70_477_993,
        source_type: SourceType::Wikidata,
        name: "LabSpec spectrometer",
        extensions: &["spc"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
