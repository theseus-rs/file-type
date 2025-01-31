use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857917: FileFormat = FileFormat {
    id: 105_857_917,
    puid: "wikidata/105857917",
    name: "SL9821 Hard disk image",
    extensions: &["slh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x44, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
