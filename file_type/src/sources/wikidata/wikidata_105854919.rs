use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854919: FileFormat = FileFormat {
    id: 105_854_919,
    source_type: SourceType::Wikidata,
    name: "ArCon project",
    extensions: &["acp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x72, 0x43, 0x6F, 0x6E, 0x20, 0x2D, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65,
                    0x6B, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
