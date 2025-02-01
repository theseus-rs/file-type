use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854946: FileFormat = FileFormat {
    id: 105_854_946,
    puid: "wikidata/105854946",
    name: "CrossePAC compressed archive",
    extensions: &["pac"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x49, 0x47, 0x44, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
