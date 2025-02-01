use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861152: FileFormat = FileFormat {
    id: 105_861_152,
    puid: "wikidata/105861152",
    name: "Mythos Software LIB game data container",
    extensions: &["lib"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x49, 0x42, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
