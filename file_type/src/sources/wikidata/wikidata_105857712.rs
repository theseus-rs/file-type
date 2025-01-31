use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857712: FileFormat = FileFormat {
    id: 105_857_712,
    puid: "wikidata/105857712",
    name: "IT8.7/1 target descriptor",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x54, 0x38, 0x2E, 0x37, 0x2F, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
