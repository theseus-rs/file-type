use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866098: FileFormat = FileFormat {
    id: 105_866_098,
    source_type: SourceType::Wikidata,
    name: "Poker Programming Language",
    extensions: &["ppl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x88, 0x62, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
