use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119966628: FileFormat = FileFormat {
    id: 119_966_628,
    source_type: SourceType::Wikidata,
    name: "Pocket Streets Map",
    extensions: &["mps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
