use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129167288: FileType = FileType {
    file_format: &FileFormat {
        id: 129_167_288,
        source_type: SourceType::Wikidata,
        name: "execline file format",
        extensions: &["exec"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
