use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854908: FileFormat = FileFormat {
    id: 105_854_908,
    source_type: SourceType::Wikidata,
    name: "Compressed Square Wave (v2.0)",
    extensions: &["csw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x53, 0x71,
                    0x75, 0x61, 0x72, 0x65, 0x20, 0x57, 0x61, 0x76, 0x65, 0x1A, 0x02, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
