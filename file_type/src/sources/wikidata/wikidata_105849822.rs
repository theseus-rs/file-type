use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849822: FileFormat = FileFormat {
    id: 105_849_822,
    puid: "wikidata/105849822",
    name: "Painter 3D Contour",
    extensions: &["cnt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6E, 0x74, 0x0A, 0x41, 0x6C, 0x6C, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
