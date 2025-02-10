use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117313902: FileType = FileType {
    file_format: &FileFormat {
        id: 117_313_902,
        source_type: SourceType::Wikidata,
        name: "Clear Text CGM",
        extensions: &["ctm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
