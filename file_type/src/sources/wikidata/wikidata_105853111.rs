use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853111: FileFormat = FileFormat {
    id: 105_853_111,
    source_type: SourceType::Wikidata,
    name: "StarWriter 2.x Document",
    extensions: &["sdw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x57, 0x47, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
