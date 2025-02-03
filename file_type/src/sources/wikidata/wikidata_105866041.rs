use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866041: FileFormat = FileFormat {
    id: 105_866_041,
    source_type: SourceType::Wikidata,
    name: "Lotus Picture",
    extensions: &["pic"],
    media_types: &["image/x-lotus-pic"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0x00, 0x44, 0x00, 0x00, 0x00, 0x00,
                    0x0C, 0x7F, 0x09, 0x06,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
