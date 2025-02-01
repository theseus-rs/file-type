use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852441: FileFormat = FileFormat {
    id: 105_852_441,
    puid: "wikidata/105852441",
    name: "A'dam Music Composer Score",
    extensions: &["sco"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x6D, 0x38, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
