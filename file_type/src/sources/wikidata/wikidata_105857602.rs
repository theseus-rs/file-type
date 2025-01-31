use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857602: FileFormat = FileFormat {
    id: 105_857_602,
    puid: "wikidata/105857602",
    name: "G64 textual representation disk image",
    extensions: &["txt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6E, 0x6F, 0x2D, 0x74, 0x72, 0x61, 0x63, 0x6B, 0x73, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
