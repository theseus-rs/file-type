use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854593: FileFormat = FileFormat {
    id: 105_854_593,
    source_type: SourceType::Wikidata,
    name: "SKYT/Drifters Packer song",
    extensions: &["skyt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4B, 0x59, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
