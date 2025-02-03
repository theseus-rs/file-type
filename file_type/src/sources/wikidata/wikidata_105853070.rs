use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853070: FileFormat = FileFormat {
    id: 105_853_070,
    source_type: SourceType::Wikidata,
    name: "Linux/UNIX shell script",
    extensions: &["sh"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21, 0x2F, 0x62, 0x69, 0x6E, 0x2F])],
            },
        }],
    }],
    related_formats: &[],
};
