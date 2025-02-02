use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855468: FileFormat = FileFormat {
    id: 105_855_468,
    source_type: SourceType::Wikidata,
    name: "FreePCB project",
    extensions: &["fpc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x6F, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x73, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
