use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862259: FileFormat = FileFormat {
    id: 105_862_259,
    puid: "wikidata/105862259",
    name: "Video Edit Magic project",
    extensions: &["mpj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x74, 0x69, 0x6D, 0x65, 0x6C, 0x69, 0x6E, 0x65, 0x3E, 0x0D, 0x0A, 0x09,
                    0x3C, 0x67, 0x72, 0x6F, 0x75, 0x70, 0x20, 0x74, 0x79, 0x70, 0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
