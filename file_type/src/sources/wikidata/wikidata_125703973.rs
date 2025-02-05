use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125703973: FileFormat = FileFormat {
    id: 125_703_973,
    source_type: SourceType::Wikidata,
    name: "StarWriter 4.0/5.0 Master Document",
    extensions: &["sgl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
