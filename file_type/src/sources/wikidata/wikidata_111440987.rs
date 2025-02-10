use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111440987: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_987,
        source_type: SourceType::Wikidata,
        name: "Visual Basic UserDocument",
        extensions: &["dob"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
