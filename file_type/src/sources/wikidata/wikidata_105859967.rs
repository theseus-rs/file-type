use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859967: FileFormat = FileFormat {
    id: 105_859_967,
    source_type: SourceType::Wikidata,
    name: "Visual Boy Advance movie capture",
    extensions: &["vbm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x42, 0x4D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
