use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849765: FileFormat = FileFormat {
    id: 105_849_765,
    source_type: SourceType::Wikidata,
    name: "Internet Security Certificate",
    extensions: &["cer"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2D, 0x2D, 0x2D, 0x2D, 0x2D, 0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x43, 0x45,
                    0x52, 0x54, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x45, 0x2D, 0x2D, 0x2D, 0x2D,
                    0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
