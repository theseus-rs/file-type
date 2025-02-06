use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856885: FileFormat = FileFormat {
    id: 105_856_885,
    source_type: SourceType::Wikidata,
    name: "Galaxkey encrypted data",
    extensions: &["gxk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x47, 0x53, 0x53, 0x23, 0x03, 0x00, 0x00, 0x00, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
