use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860166: FileFormat = FileFormat {
    id: 105_860_166,
    puid: "wikidata/105860166",
    name: "REALbasic Form/Window",
    extensions: &["rbfrm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x74, 0x61, 0x67, 0x20, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
