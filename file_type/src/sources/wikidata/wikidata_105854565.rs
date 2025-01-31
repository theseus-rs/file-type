use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854565: FileFormat = FileFormat {
    id: 105_854_565,
    puid: "wikidata/105854565",
    name: "B6Zip compressed archive",
    extensions: &["b6z"],
    media_types: &["application/x-b6z-compressed"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x36, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
