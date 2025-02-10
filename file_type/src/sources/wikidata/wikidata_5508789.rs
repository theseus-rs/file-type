use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5508789: FileType = FileType {
    file_format: &FileFormat {
        id: 5_508_789,
        source_type: SourceType::Wikidata,
        name: "Functional Mock-up Interface",
        extensions: &["fmu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
