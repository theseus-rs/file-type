use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967199: FileFormat = FileFormat {
    id: 27_967_199,
    source_type: SourceType::Wikidata,
    name: "Liquid Tracker module",
    extensions: &["liq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
