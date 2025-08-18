use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135258592: FileType = FileType {
    file_format: &FileFormat {
        id: 135_258_592,
        source_type: SourceType::Wikidata,
        name: "Beanshell script",
        extensions: &["bsh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
