use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856937: FileFormat = FileFormat {
    id: 105_856_937,
    source_type: SourceType::Wikidata,
    name: "Greenfoot Project",
    extensions: &["greenfoot"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x47, 0x72, 0x65, 0x65, 0x6E, 0x66, 0x6F, 0x6F, 0x74, 0x20, 0x70, 0x72,
                    0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
