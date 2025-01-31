use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858220: FileFormat = FileFormat {
    id: 105_858_220,
    puid: "wikidata/105858220",
    name: "Encore Musical Notation",
    extensions: &["enc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
