use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856150: FileFormat = FileFormat {
    id: 105_856_150,
    source_type: SourceType::Wikidata,
    name: "Project Dogwaffle animation (type 1)",
    extensions: &["dwa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x57, 0x41, 0x31, 0x20, 0x20, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
