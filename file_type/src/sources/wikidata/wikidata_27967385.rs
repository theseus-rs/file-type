use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967385: FileFormat = FileFormat {
    id: 27_967_385,
    source_type: SourceType::Wikidata,
    name: "Extended MIDI",
    extensions: &["xmi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
