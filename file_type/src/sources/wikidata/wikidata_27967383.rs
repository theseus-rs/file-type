use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967383: FileFormat = FileFormat {
    id: 27_967_383,
    source_type: SourceType::Wikidata,
    name: "RIFF MIDI",
    extensions: &["rmi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
