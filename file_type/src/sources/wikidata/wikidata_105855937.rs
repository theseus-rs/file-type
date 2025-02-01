use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855937: FileFormat = FileFormat {
    id: 105_855_937,
    puid: "wikidata/105855937",
    name: "Dave 2 Huffman compressed game data",
    extensions: &["dd2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x55, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
