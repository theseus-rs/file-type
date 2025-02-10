use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122403904: FileType = FileType {
    file_format: &FileFormat {
        id: 122_403_904,
        source_type: SourceType::Wikidata,
        name: "CodeWarrior Constructor Resource File",
        extensions: &["rsr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
