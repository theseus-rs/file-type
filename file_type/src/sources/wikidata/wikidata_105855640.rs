use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855640: FileFormat = FileFormat {
    id: 105_855_640,
    source_type: SourceType::Wikidata,
    name: "Opticks Object",
    extensions: &["obj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x4F, 0x42, 0x4A, 0x45, 0x43, 0x54, 0x76, 0x31, 0x2E, 0x30, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
