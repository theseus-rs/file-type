use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137556315: FileType = FileType {
    file_format: &FileFormat {
        id: 137_556_315,
        source_type: SourceType::Wikidata,
        name: "JMulTi data file",
        extensions: &["dta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
