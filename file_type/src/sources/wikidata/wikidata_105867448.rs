use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867448: FileFormat = FileFormat {
    id: 105_867_448,
    puid: "wikidata/105867448",
    name: "Seifert ASCII pole figure format",
    extensions: &["nja"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x4D, 0x65, 0x61, 0x73, 0x50, 0x61, 0x72, 0x61, 0x6D, 0x65, 0x74, 0x65,
                    0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
