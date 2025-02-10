use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919071: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_071,
        source_type: SourceType::Wikidata,
        name: "After Effects project, binary variant",
        extensions: &["aep"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
