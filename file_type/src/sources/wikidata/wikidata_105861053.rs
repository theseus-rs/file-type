use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861053: FileFormat = FileFormat {
    id: 105_861_053,
    source_type: SourceType::Wikidata,
    name: "Lineage II data",
    extensions: &["l2r"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x65, 0x00, 0x61, 0x00, 0x67, 0x00, 0x65,
                    0x00, 0x32, 0x00, 0x56, 0x00, 0x65, 0x00, 0x72, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
