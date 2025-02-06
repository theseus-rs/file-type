use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854002: FileFormat = FileFormat {
    id: 105_854_002,
    source_type: SourceType::Wikidata,
    name: "Anfy Applet Generator Saved file",
    extensions: &["ajp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x20, 0x4A, 0x20, 0x50, 0x20, 0x31, 0x20, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
