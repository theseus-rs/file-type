use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116784988: FileType = FileType {
    file_format: &FileFormat {
        id: 116_784_988,
        source_type: SourceType::Wikidata,
        name: "Project Manager Pro Template file",
        extensions: &["pmt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
