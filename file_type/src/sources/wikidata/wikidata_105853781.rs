use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853781: FileFormat = FileFormat {
    id: 105_853_781,
    source_type: SourceType::Wikidata,
    name: "JRchive compressed archive",
    extensions: &["jrc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x52, 0x63, 0x68, 0x69, 0x76, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
