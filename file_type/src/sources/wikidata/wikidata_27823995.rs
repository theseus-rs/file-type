use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27823995: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_995,
        source_type: SourceType::Wikidata,
        name: "Maptech KAPP image file, version 3.0",
        extensions: &["kap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
