use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205458: FileFormat = FileFormat {
    id: 28_205_458,
    source_type: SourceType::Wikidata,
    name: "PCO B16",
    extensions: &["b16"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x4F, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
