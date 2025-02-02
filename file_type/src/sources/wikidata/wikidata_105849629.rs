use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849629: FileFormat = FileFormat {
    id: 105_849_629,
    source_type: SourceType::Wikidata,
    name: "CCS64 snapshot (v1)",
    extensions: &["c64"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x00, 0x00, 0x00, 0x37, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
