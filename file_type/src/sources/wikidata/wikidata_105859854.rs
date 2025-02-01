use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859854: FileFormat = FileFormat {
    id: 105_859_854,
    puid: "wikidata/105859854",
    name: "SymbOS VID video",
    extensions: &["vid"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x79, 0x6D, 0x56, 0x69, 0x64, 0x31, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
