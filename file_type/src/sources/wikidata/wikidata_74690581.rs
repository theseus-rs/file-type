use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74690581: FileFormat = FileFormat {
    id: 74_690_581,
    source_type: SourceType::Wikidata,
    name: "TomeRaider e-book/document",
    extensions: &["tr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x37, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0xD2, 0x16, 0x00, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
