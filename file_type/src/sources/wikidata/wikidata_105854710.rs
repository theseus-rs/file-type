use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854710: FileFormat = FileFormat {
    id: 105_854_710,
    source_type: SourceType::Wikidata,
    name: "mcm compressed archive",
    extensions: &["mcm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x43, 0x4D, 0x41, 0x52, 0x43, 0x48, 0x49, 0x56, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
