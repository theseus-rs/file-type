use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131453612: FileFormat = FileFormat {
    id: 131_453_612,
    source_type: SourceType::Wikidata,
    name: "Zeek file format",
    extensions: &["bro", "zeek"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
