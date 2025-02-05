use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_19860869: FileFormat = FileFormat {
    id: 19_860_869,
    source_type: SourceType::Wikidata,
    name: "Itinerary file",
    extensions: &["itn"],
    media_types: &["application/itn"],
    signatures: &[],
    related_formats: &[],
};
