use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860342: FileFormat = FileFormat {
    id: 105_860_342,
    puid: "wikidata/105860342",
    name: "BIS Rtm animation",
    extensions: &["rtm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x54, 0x4D, 0x5F, 0x30, 0x31, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
