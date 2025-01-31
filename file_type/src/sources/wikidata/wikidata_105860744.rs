use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860744: FileFormat = FileFormat {
    id: 105_860_744,
    puid: "wikidata/105860744",
    name: "MSX2 ROM Image",
    extensions: &["rom"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
