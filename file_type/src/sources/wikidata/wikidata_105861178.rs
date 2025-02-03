use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861178: FileFormat = FileFormat {
    id: 105_861_178,
    source_type: SourceType::Wikidata,
    name: "PGE Extendable Level",
    extensions: &["lvlx"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x45, 0x41, 0x44, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
