use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967087: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_087,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts AS4/ASF Music",
        extensions: &["as4", "asf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
