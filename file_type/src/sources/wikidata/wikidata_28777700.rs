use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28777700: FileFormat = FileFormat {
    id: 28_777_700,
    source_type: SourceType::Wikidata,
    name: "Mozilla Archive",
    extensions: &["mar"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x52, 0x31, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
