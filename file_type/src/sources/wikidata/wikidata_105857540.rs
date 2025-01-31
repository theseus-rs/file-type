use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857540: FileFormat = FileFormat {
    id: 105_857_540,
    puid: "wikidata/105857540",
    name: "SMSQ/E hard disk image",
    extensions: &["win"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4C, 0x57, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
