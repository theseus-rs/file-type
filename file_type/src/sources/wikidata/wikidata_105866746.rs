use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866746: FileFormat = FileFormat {
    id: 105_866_746,
    source_type: SourceType::Wikidata,
    name: "PROGRESS Procedure Library (v9)",
    extensions: &["pl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD7, 0x07, 0x07])],
            },
        }],
    }],
    related_formats: &[],
};
