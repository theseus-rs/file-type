use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858182: FileFormat = FileFormat {
    id: 105_858_182,
    puid: "wikidata/105858182",
    name: "GP32 Free eXecutable Encrypted",
    extensions: &["fxe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x78, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
