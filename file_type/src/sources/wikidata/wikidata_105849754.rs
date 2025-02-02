use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849754: FileFormat = FileFormat {
    id: 105_849_754,
    source_type: SourceType::Wikidata,
    name: "CARA 3D group object",
    extensions: &["csy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x54, 0x53, 0x00, 0x43, 0x53, 0x59, 0x00, 0x78, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
