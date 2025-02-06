use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128628799: FileFormat = FileFormat {
    id: 128_628_799,
    source_type: SourceType::Wikidata,
    name: "BARE schema source",
    extensions: &["bare"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
