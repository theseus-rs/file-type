use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855211: FileFormat = FileFormat {
    id: 105_855_211,
    source_type: SourceType::Wikidata,
    name: "FormWorx for Windows Form (v2.x)",
    extensions: &["fpx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x02, 0x00, 0x10, 0x04, 0x1A, 0x00, 0x02, 0x00, 0x00, 0x00, 0x1A,
                    0x89, 0x02, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
