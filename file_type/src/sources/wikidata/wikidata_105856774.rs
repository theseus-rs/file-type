use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856774: FileFormat = FileFormat {
    id: 105_856_774,
    source_type: SourceType::Wikidata,
    name: "Universal Classification Standard Database",
    extensions: &["ucs"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x55, 0x43, 0x53, 0x57, 0x44, 0x20, 0x4C, 0x41, 0x4E, 0x47, 0x55, 0x41,
                    0x47, 0x45, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
