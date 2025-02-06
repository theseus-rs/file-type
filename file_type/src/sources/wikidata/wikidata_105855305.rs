use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855305: FileFormat = FileFormat {
    id: 105_855_305,
    source_type: SourceType::Wikidata,
    name: "Fritzing sketch",
    extensions: &["fz"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
