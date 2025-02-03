use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851877: FileFormat = FileFormat {
    id: 105_851_877,
    source_type: SourceType::Wikidata,
    name: "Speedy System module",
    extensions: &["ss"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x50, 0x45, 0x45, 0x44, 0x59, 0x2D, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
