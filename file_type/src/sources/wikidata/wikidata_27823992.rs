use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27823992: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_992,
        source_type: SourceType::Wikidata,
        name: "Maptech BSB documentation file, version 3.0",
        extensions: &["bsb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
