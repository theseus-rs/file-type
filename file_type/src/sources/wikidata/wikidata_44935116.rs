use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_44935116: FileType = FileType {
    file_format: &FileFormat {
        id: 44_935_116,
        source_type: SourceType::Wikidata,
        name: "MS-DOS text file",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
