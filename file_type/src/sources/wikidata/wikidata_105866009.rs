use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866009: FileFormat = FileFormat {
    id: 105_866_009,
    puid: "wikidata/105866009",
    name: "Aerofly model parameters",
    extensions: &["par"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
