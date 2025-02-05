use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110502531: FileFormat = FileFormat {
    id: 110_502_531,
    source_type: SourceType::Wikidata,
    name: "ISDOCX Information System Document (Generic)",
    extensions: &["isdocx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
