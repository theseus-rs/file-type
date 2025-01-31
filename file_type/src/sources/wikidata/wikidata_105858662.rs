use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858662: FileFormat = FileFormat {
    id: 105_858_662,
    puid: "wikidata/105858662",
    name: "Blue Byte Archive Format",
    extensions: &["bba"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x41, 0x46, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
