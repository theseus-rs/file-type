use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4052556: FileType = FileType {
    file_format: &FileFormat {
        id: 4_052_556,
        source_type: SourceType::Wikidata,
        name: "Vector Quantized Animation",
        extensions: &["vqa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
