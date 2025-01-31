use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849998: FileFormat = FileFormat {
    id: 105_849_998,
    puid: "wikidata/105849998",
    name: "Visual Basic class definition",
    extensions: &["cls"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x31, 0x2E, 0x30, 0x20, 0x43,
                    0x4C, 0x41, 0x53, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
