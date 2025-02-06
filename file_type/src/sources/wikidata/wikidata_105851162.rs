use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851162: FileFormat = FileFormat {
    id: 105_851_162,
    source_type: SourceType::Wikidata,
    name: "Trelby document",
    extensions: &["trelby"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x23, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
