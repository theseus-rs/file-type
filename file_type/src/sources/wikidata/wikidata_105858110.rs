use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858110: FileFormat = FileFormat {
    id: 105_858_110,
    puid: "wikidata/105858110",
    name: "EM400 disk image",
    extensions: &["e4i"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x34, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
