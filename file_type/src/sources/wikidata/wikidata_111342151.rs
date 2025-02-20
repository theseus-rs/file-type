use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111342151: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_151,
        source_type: SourceType::Wikidata,
        name: "J-Phone / SmdEd mobile song",
        extensions: &["smd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
