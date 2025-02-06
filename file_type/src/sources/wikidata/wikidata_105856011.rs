use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856011: FileFormat = FileFormat {
    id: 105_856_011,
    source_type: SourceType::Wikidata,
    name: "DGIndex project",
    extensions: &["d2v"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x47, 0x49, 0x6E, 0x64, 0x65, 0x78, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63,
                    0x74, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
