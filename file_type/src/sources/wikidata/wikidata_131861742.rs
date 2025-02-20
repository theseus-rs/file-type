use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131861742: FileType = FileType {
    file_format: &FileFormat {
        id: 131_861_742,
        source_type: SourceType::Wikidata,
        name: "GE Signa ximg file",
        extensions: &["ximg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
