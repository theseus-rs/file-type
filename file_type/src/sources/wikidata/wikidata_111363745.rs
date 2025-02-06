use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363745: FileFormat = FileFormat {
    id: 111_363_745,
    source_type: SourceType::Wikidata,
    name: "Miles Sound System extended MIDI file",
    extensions: &["xmi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
