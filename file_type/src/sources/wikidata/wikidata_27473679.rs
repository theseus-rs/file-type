use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27473679: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_679,
        source_type: SourceType::Wikidata,
        name: "Band Sequential Image File",
        extensions: &["bsq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
