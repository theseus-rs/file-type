use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851641: FileFormat = FileFormat {
    id: 105_851_641,
    source_type: SourceType::Wikidata,
    name: "Xara resources index",
    extensions: &["txt"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x49, 0x6E, 0x64, 0x65, 0x78, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                    0x66, 0x6F, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
