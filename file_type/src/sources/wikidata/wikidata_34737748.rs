use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34737748: FileType = FileType {
    file_format: &FileFormat {
        id: 34_737_748,
        source_type: SourceType::Wikidata,
        name: "Snappy framing format",
        extensions: &["sz"],
        media_types: &["application/x-snappy-framed"],
        signatures: &[],
        related_formats: &[],
    },
};
