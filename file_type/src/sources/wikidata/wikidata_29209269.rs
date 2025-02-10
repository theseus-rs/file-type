use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29209269: FileType = FileType {
    file_format: &FileFormat {
        id: 29_209_269,
        source_type: SourceType::Wikidata,
        name: "Z",
        extensions: &["z"],
        media_types: &["application/x-compress"],
        signatures: &[],
        related_formats: &[],
    },
};
