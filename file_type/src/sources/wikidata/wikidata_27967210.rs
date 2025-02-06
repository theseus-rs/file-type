use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967210: FileFormat = FileFormat {
    id: 27_967_210,
    source_type: SourceType::Wikidata,
    name: "Poly Tracker module",
    extensions: &["ptm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
