use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861144: FileFormat = FileFormat {
    id: 105_861_144,
    source_type: SourceType::Wikidata,
    name: "Liko-12 disk",
    extensions: &["lk12"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4B, 0x31, 0x32, 0x3B])],
            },
        }],
    }],
    related_formats: &[],
};
