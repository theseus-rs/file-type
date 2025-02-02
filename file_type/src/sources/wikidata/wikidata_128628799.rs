use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128628799: FileFormat = FileFormat {
    id: 128_628_799,
    source_type: SourceType::Wikidata,
    name: "BARE schema source",
    extensions: &["bare"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
