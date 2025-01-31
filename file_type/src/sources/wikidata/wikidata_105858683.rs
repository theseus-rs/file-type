use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858683: FileFormat = FileFormat {
    id: 105_858_683,
    puid: "wikidata/105858683",
    name: "True Colour Picture bitmap",
    extensions: &["trp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x74, 0x72, 0x75, 0x3F])],
            },
        }],
    }],
    related_formats: &[],
};
