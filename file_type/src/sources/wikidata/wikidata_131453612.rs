use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131453612: FileFormat = FileFormat {
    id: 131_453_612,
    source_type: SourceType::Wikidata,
    name: "Zeek file format",
    extensions: &["bro", "zeek"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
