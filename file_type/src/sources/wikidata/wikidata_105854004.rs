use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854004: FileFormat = FileFormat {
    id: 105_854_004,
    source_type: SourceType::Wikidata,
    name: "Allegro Packfile (compressed)",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x6C, 0x68, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
