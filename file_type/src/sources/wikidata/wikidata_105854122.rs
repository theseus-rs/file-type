use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854122: FileFormat = FileFormat {
    id: 105_854_122,
    source_type: SourceType::Wikidata,
    name: "ROFF 3D animation",
    extensions: &["rof"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4F, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
