use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855974: FileFormat = FileFormat {
    id: 105_855_974,
    source_type: SourceType::Wikidata,
    name: "Borland Delphi - C++ Builder Form (var.3)",
    extensions: &["dfm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x0A, 0x00, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
