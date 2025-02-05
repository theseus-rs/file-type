use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854651: FileFormat = FileFormat {
    id: 105_854_651,
    source_type: SourceType::Wikidata,
    name: "Atlantis Word Processor SpellCheck settings",
    extensions: &["asc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x47, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x6C, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
