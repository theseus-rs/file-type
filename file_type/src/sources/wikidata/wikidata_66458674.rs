use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66458674: FileType = FileType {
    file_format: &FileFormat {
        id: 66_458_674,
        source_type: SourceType::Wikidata,
        name: "Adobe Dimensions file format",
        extensions: &["dim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
