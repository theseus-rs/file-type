use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34747117: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_117,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Matrix File",
        extensions: &["smx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
