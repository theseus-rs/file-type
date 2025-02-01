use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862170: FileFormat = FileFormat {
    id: 105_862_170,
    puid: "wikidata/105862170",
    name: "Project Dogwaffle mixing palette (type 2)",
    extensions: &["mix"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x49, 0x58, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
