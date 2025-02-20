use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009267: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_267,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Newsletter File format",
        extensions: &["nws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
