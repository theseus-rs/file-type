use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049418: FileFormat = FileFormat {
    id: 28_049_418,
    source_type: SourceType::Wikidata,
    name: "DEGAS image, high resolution",
    extensions: &["PI3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
