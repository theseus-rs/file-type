use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967216: FileFormat = FileFormat {
    id: 27_967_216,
    source_type: SourceType::Wikidata,
    name: "SCC Blaffer NT instrument kit",
    extensions: &["sbk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x6C, 0x61, 0x66, 0x20, 0x4E, 0x54, 0x20, 0x49, 0x6E, 0x73, 0x20, 0x4B,
                    0x69, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
