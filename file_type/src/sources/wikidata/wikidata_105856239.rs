use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856239: FileFormat = FileFormat {
    id: 105_856_239,
    source_type: SourceType::Wikidata,
    name: "Borland Delphi - C++ Builder Form (var.1)",
    extensions: &["dfm"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
