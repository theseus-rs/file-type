use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864897: FileFormat = FileFormat {
    id: 105_864_897,
    source_type: SourceType::Wikidata,
    name: "Pepakura Designer work (with rem)",
    extensions: &["pdo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x50, 0x65, 0x70, 0x61, 0x6B, 0x75, 0x72, 0x61, 0x20, 0x44, 0x65,
                    0x73, 0x69, 0x67, 0x6E, 0x65, 0x72, 0x20, 0x57, 0x6F, 0x72, 0x6B, 0x20, 0x49,
                    0x6E, 0x66, 0x6F, 0x20, 0x76, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
