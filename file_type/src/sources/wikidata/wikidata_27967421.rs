use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967421: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_421,
        source_type: SourceType::Wikidata,
        name: "CapXML",
        extensions: &["capx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
