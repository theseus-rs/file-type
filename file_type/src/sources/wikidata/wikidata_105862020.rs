use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862020: FileFormat = FileFormat {
    id: 105_862_020,
    source_type: SourceType::Wikidata,
    name: "Descent Mission",
    extensions: &["msn"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x20, 0x3D, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
