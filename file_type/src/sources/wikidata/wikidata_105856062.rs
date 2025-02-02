use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856062: FileFormat = FileFormat {
    id: 105_856_062,
    source_type: SourceType::Wikidata,
    name: "Her Interactive game data archive",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x49, 0x46, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x20, 0x48, 0x65, 0x72, 0x49,
                    0x6E, 0x74, 0x65, 0x72, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
