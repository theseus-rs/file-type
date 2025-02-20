use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130062600: FileType = FileType {
    file_format: &FileFormat {
        id: 130_062_600,
        source_type: SourceType::Wikidata,
        name: "Kal source code file",
        extensions: &["kal"],
        media_types: &["application/kal", "text/kal"],
        signatures: &[],
        related_formats: &[],
    },
};
