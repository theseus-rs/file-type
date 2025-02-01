use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854127: FileFormat = FileFormat {
    id: 105_854_127,
    puid: "wikidata/105854127",
    name: "Alpha Five Web Components",
    extensions: &["a5wcmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x35, 0x42, 0x4C, 0x4F, 0x42, 0x53, 0x54, 0x00, 0x02, 0x00, 0x00, 0x1C,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
