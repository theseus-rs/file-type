use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856506: FileFormat = FileFormat {
    id: 105_856_506,
    source_type: SourceType::Wikidata,
    name: "EXP document",
    extensions: &["wxp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4C, 0x53, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
