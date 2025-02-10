use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117851210: FileType = FileType {
    file_format: &FileFormat {
        id: 117_851_210,
        source_type: SourceType::Wikidata,
        name: "Tektronix Plot 10 file",
        extensions: &["p10"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
