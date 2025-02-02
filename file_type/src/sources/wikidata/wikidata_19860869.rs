use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_19860869: FileFormat = FileFormat {
    id: 19_860_869,
    source_type: SourceType::Wikidata,
    name: "Itinerary file",
    extensions: &["itn"],
    media_types: &["application/itn"],
    internal_signatures: &[],
    related_formats: &[],
};
