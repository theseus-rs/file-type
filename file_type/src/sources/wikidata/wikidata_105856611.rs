use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856611: FileFormat = FileFormat {
    id: 105_856_611,
    source_type: SourceType::Wikidata,
    name: "WebMSX Save State",
    extensions: &["wst"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x77, 0x6D, 0x73, 0x78, 0x00, 0x00, 0x73, 0x74, 0x61, 0x74, 0x65,
                    0x21, 0x7B, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
