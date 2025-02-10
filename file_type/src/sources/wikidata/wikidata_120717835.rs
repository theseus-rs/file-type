use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120717835: FileType = FileType {
    file_format: &FileFormat {
        id: 120_717_835,
        source_type: SourceType::Wikidata,
        name: "DeductionPro 2006 Data File",
        extensions: &["d06"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
