use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855910: FileFormat = FileFormat {
    id: 105_855_910,
    source_type: SourceType::Wikidata,
    name: "Borland Delphi - C++ Builder Form (var.2)",
    extensions: &["dfm"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x6E, 0x68, 0x65, 0x72, 0x69, 0x74, 0x65, 0x64, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
