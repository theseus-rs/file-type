use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967389: FileFormat = FileFormat {
    id: 27_967_389,
    source_type: SourceType::Wikidata,
    name: "Adlib Tracker module",
    extensions: &["sng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
