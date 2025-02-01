use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866261: FileFormat = FileFormat {
    id: 105_866_261,
    puid: "wikidata/105866261",
    name: "PipeDream document",
    extensions: &["pd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x4F, 0x50, 0x25])],
            },
        }],
    }],
    related_formats: &[],
};
