use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123296707: FileFormat = FileFormat {
    id: 123_296_707,
    source_type: SourceType::Wikidata,
    name: "CD-Face Layout",
    extensions: &["ntp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
