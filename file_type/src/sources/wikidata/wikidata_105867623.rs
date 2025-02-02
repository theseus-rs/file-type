use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867623: FileFormat = FileFormat {
    id: 105_867_623,
    source_type: SourceType::Wikidata,
    name: "PlayStation 3 NPDRM encrypted SDATA",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x50, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
