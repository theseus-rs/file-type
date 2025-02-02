use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852116: FileFormat = FileFormat {
    id: 105_852_116,
    source_type: SourceType::Wikidata,
    name: "Dirk Bialluch TPU samples",
    extensions: &["smp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x4D, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
