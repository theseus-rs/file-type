use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862118: FileFormat = FileFormat {
    id: 105_862_118,
    source_type: SourceType::Wikidata,
    name: "Audio Interface Library 3 Music/MIDI driver",
    extensions: &["mdi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x49, 0x4C, 0x33, 0x4D, 0x44, 0x49, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
