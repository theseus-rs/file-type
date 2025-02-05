use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851771: FileFormat = FileFormat {
    id: 105_851_771,
    source_type: SourceType::Wikidata,
    name: "AY STRC chiptune",
    extensions: &["strc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x58, 0x41, 0x59, 0x53, 0x54, 0x52, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
