use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87066063: FileFormat = FileFormat {
    id: 87_066_063,
    puid: "wikidata/87066063",
    name: "LEADTools Lead 1Bit Compressed Image",
    extensions: &["cmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x54, 0x52, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
