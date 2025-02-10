use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118640353: FileType = FileType {
    file_format: &FileFormat {
        id: 118_640_353,
        source_type: SourceType::Wikidata,
        name: "Picture Definition file",
        extensions: &["lpd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
