use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849836: FileFormat = FileFormat {
    id: 105_849_836,
    source_type: SourceType::Wikidata,
    name: "CleWin CIF layout",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x43, 0x49, 0x46, 0x20, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6E, 0x20,
                    0x62, 0x79, 0x20, 0x43, 0x6C, 0x65, 0x57, 0x69, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
