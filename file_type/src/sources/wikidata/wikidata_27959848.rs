use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959848: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_848,
        source_type: SourceType::Wikidata,
        name: "LMMS music file",
        extensions: &["mmp", "mmpz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
