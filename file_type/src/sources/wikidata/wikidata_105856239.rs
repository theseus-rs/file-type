use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856239: FileFormat = FileFormat {
    id: 105_856_239,
    puid: "wikidata/105856239",
    name: "Borland Delphi - C++ Builder Form (var.1)",
    extensions: &["dfm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
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
