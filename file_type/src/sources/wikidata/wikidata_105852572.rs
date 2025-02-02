use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852572: FileFormat = FileFormat {
    id: 105_852_572,
    source_type: SourceType::Wikidata,
    name: "SETI@Home Classic results",
    extensions: &["sah"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6F, 0x67, 0x68, 0x20, 0x6E, 0x63, 0x66, 0x66, 0x74, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
