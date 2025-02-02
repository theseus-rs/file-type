use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849691: FileFormat = FileFormat {
    id: 105_849_691,
    source_type: SourceType::Wikidata,
    name: "CodeSuite DataBase - BitMatch",
    extensions: &["cdb"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x50, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x3E, 0x42,
                    0x69, 0x74, 0x4D, 0x61, 0x74, 0x63, 0x68,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
