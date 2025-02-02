use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856399: FileFormat = FileFormat {
    id: 105_856_399,
    source_type: SourceType::Wikidata,
    name: "WorldMachine document",
    extensions: &["tmd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x50, 0x52, 0x57, 0x6F, 0x72, 0x6C, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
