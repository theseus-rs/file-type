use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852232: FileFormat = FileFormat {
    id: 105_852_232,
    puid: "wikidata/105852232",
    name: "Strand Games save game",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x32, 0x53, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
