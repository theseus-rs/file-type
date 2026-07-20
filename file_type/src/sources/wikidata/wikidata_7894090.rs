use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7894090: FileType = FileType {
    file_format: &FileFormat {
        id: 7_894_090,
        source_type: SourceType::Wikidata,
        name: "Universal Terminology eXchange",
        extensions: &["utx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
