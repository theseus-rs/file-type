use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136464298: FileType = FileType {
    file_format: &FileFormat {
        id: 136_464_298,
        source_type: SourceType::Wikidata,
        name: "PyTorch Serialized File Format",
        extensions: &["pt", "pth"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
