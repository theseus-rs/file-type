use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850509: FileFormat = FileFormat {
    id: 105_850_509,
    source_type: SourceType::Wikidata,
    name: "EasyCalc spreadsheet (v1.0)",
    extensions: &["calc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x41, 0x53, 0x59, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
