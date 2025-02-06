use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967223: FileFormat = FileFormat {
    id: 27_967_223,
    source_type: SourceType::Wikidata,
    name: "StarTrekker/Star Tracker module",
    extensions: &["mod", "nt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
