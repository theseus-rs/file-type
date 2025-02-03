use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862480: FileFormat = FileFormat {
    id: 105_862_480,
    source_type: SourceType::Wikidata,
    name: "MIDI-MAZE II Maze",
    extensions: &["mze"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE4, 0x4D, 0x5A, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
