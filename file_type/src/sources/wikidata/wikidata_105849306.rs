use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849306: FileFormat = FileFormat {
    id: 105_849_306,
    source_type: SourceType::Wikidata,
    name: "TrainController Animation",
    extensions: &["yra"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0xFE, 0xCA])],
            },
        }],
    }],
    related_formats: &[],
};
