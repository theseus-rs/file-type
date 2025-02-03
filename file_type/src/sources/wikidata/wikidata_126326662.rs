use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126326662: FileFormat = FileFormat {
    id: 126_326_662,
    source_type: SourceType::Wikidata,
    name: "slrn configuration file",
    extensions: &["slrnrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
