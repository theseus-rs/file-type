use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861727: FileFormat = FileFormat {
    id: 105_861_727,
    puid: "wikidata/105861727",
    name: "Digital Tracker 6-channel module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x41, 0x30, 0x36])],
            },
        }],
    }],
    related_formats: &[],
};
