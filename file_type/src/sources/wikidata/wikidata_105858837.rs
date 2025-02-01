use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858837: FileFormat = FileFormat {
    id: 105_858_837,
    puid: "wikidata/105858837",
    name: "Chess Assistant comments Bodies",
    extensions: &["bdy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6D, 0x6D, 0x65, 0x6E, 0x74, 0x73, 0x20, 0x62, 0x6F, 0x64, 0x69,
                    0x65, 0x73, 0x2E, 0x20, 0x43, 0x68, 0x65, 0x73, 0x73, 0x20, 0x41, 0x73, 0x73,
                    0x69, 0x73, 0x74, 0x61, 0x6E, 0x74, 0x20, 0x56, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
