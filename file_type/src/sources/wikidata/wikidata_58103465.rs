use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58103465: FileType = FileType {
    file_format: &FileFormat {
        id: 58_103_465,
        source_type: SourceType::Wikidata,
        name: "Adobe Font List",
        extensions: &["lst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
