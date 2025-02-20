use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_65533101: FileType = FileType {
    file_format: &FileFormat {
        id: 65_533_101,
        source_type: SourceType::Wikidata,
        name: "MealPlan file format",
        extensions: &["pln"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
