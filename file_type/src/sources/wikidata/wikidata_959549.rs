use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_959549: FileType = FileType {
    file_format: &FileFormat {
        id: 959_549,
        source_type: SourceType::Wikidata,
        name: "shell script",
        extensions: &[],
        media_types: &["application/x-sh", "application/x-shellscript", "text/x-sh"],
        signatures: &[],
        related_formats: &[],
    },
};
