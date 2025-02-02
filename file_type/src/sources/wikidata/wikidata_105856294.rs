use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856294: FileFormat = FileFormat {
    id: 105_856_294,
    source_type: SourceType::Wikidata,
    name: "Bentley MicroStation CAD drawing (complex)",
    extensions: &["dgn"],
    media_types: &["application/x-bentley-dgn"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC8, 0x09, 0xFE, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
