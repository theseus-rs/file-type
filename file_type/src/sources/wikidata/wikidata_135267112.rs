use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135267112: FileType = FileType {
    file_format: &FileFormat {
        id: 135_267_112,
        source_type: SourceType::Wikidata,
        name: "FTL file",
        extensions: &["ftl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
