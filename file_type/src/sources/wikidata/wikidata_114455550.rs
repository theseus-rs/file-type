use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114455550: FileType = FileType {
    file_format: &FileFormat {
        id: 114_455_550,
        source_type: SourceType::Wikidata,
        name: "Apache Avro IDL Data",
        extensions: &["avdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
