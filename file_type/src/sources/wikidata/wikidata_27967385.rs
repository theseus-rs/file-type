use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967385: FileFormat = FileFormat {
    id: 27_967_385,
    source_type: SourceType::Wikidata,
    name: "Extended MIDI",
    extensions: &["xmi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
