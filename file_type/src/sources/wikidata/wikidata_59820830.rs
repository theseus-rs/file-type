use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59820830: FileType = FileType {
    file_format: &FileFormat {
        id: 59_820_830,
        source_type: SourceType::Wikidata,
        name: "Corel Presentation Exchange File",
        extensions: &["cmx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
