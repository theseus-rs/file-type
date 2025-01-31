use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860360: FileFormat = FileFormat {
    id: 105_860_360,
    puid: "wikidata/105860360",
    name: "Rexx-Adventure game source",
    extensions: &["rad"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x69, 0x74, 0x6C, 0x65, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
