use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6108932: FileType = FileType {
    file_format: &FileFormat {
        id: 6_108_932,
        source_type: SourceType::Wikidata,
        name: "JSGF",
        extensions: &["jsgf"],
        media_types: &["application/jsgf", "application/x-jsgf", "text/jsgf"],
        signatures: &[],
        related_formats: &[],
    },
};
