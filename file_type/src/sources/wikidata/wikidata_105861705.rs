use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861705: FileFormat = FileFormat {
    id: 105_861_705,
    source_type: SourceType::Wikidata,
    name: "Femap Model",
    extensions: &["modfem"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x9A, 0x99, 0x99, 0x99, 0x99, 0x99, 0x24, 0x40, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
