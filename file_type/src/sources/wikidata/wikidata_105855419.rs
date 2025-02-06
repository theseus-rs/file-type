use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855419: FileFormat = FileFormat {
    id: 105_855_419,
    source_type: SourceType::Wikidata,
    name: "Mechwarrior FIT data",
    extensions: &["fit"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x54, 0x69, 0x6E, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
