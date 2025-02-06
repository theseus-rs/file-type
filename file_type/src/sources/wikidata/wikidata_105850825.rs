use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850825: FileFormat = FileFormat {
    id: 105_850_825,
    source_type: SourceType::Wikidata,
    name: "Kaspersky Anti-Virus update diff",
    extensions: &["dif"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x4C, 0x44, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
