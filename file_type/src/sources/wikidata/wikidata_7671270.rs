use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7671270: FileType = FileType {
    file_format: &FileFormat {
        id: 7_671_270,
        source_type: SourceType::Wikidata,
        name: "TRANS.TBL",
        extensions: &["TBL"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
