use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856936: FileFormat = FileFormat {
    id: 105_856_936,
    source_type: SourceType::Wikidata,
    name: "Dark Forces Game data archive",
    extensions: &["gob"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4F, 0x42, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
