use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851087: FileFormat = FileFormat {
    id: 105_851_087,
    source_type: SourceType::Wikidata,
    name: "TDI Format data",
    extensions: &["txt"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x44, 0x49, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
