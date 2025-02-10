use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87765717: FileType = FileType {
    file_format: &FileFormat {
        id: 87_765_717,
        source_type: SourceType::Wikidata,
        name: "EIOffice Document",
        extensions: &["eio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
