use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854449: FileFormat = FileFormat {
    id: 105_854_449,
    puid: "wikidata/105854449",
    name: "Distribution Package archive",
    extensions: &["pac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x50, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
