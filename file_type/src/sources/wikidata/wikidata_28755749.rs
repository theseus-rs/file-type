use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28755749: FileFormat = FileFormat {
    id: 28_755_749,
    source_type: SourceType::Wikidata,
    name: "FDF",
    extensions: &["fdf"],
    media_types: &["application/vnd.fdf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x46, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
