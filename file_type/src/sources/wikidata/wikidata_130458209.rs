use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130458209: FileType = FileType {
    file_format: &FileFormat {
        id: 130_458_209,
        source_type: SourceType::Wikidata,
        name: "Pan source code file",
        extensions: &["pan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
