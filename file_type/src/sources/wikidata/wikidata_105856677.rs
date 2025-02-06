use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856677: FileFormat = FileFormat {
    id: 105_856_677,
    source_type: SourceType::Wikidata,
    name: "UNIMod created by APlayer",
    extensions: &["uni"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x50, 0x55, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
