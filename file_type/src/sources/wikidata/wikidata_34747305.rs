use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34747305: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_305,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Visual Basic Macro File",
        extensions: &["svb", "svc", "svo", "svx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
