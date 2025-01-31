use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855224: FileFormat = FileFormat {
    id: 105_855_224,
    puid: "wikidata/105855224",
    name: "WinPharoah Filter",
    extensions: &["ftr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD2, 0x0A, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
