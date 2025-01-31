use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864226: FileFormat = FileFormat {
    id: 105_864_226,
    puid: "wikidata/105864226",
    name: "PS2DIS project",
    extensions: &["pis"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x53, 0x1F])],
            },
        }],
    }],
    related_formats: &[],
};
