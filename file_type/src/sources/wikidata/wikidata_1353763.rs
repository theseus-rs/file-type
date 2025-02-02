use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1353763: FileFormat = FileFormat {
    id: 1_353_763,
    source_type: SourceType::Wikidata,
    name: "WMV HD",
    extensions: &["wmv"],
    media_types: &["video/x-ms-wmv"],
    internal_signatures: &[],
    related_formats: &[],
};
