use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856159: FileFormat = FileFormat {
    id: 105_856_159,
    source_type: SourceType::Wikidata,
    name: "Dragon UnPACKer Look",
    extensions: &["dulk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x55, 0x4C, 0x4B, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
