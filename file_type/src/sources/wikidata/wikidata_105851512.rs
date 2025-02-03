use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851512: FileFormat = FileFormat {
    id: 105_851_512,
    source_type: SourceType::Wikidata,
    name: "MegaPaint printer driver",
    extensions: &["trb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x54, 0x52, 0x42, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
