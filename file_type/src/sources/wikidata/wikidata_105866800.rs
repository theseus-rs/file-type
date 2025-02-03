use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866800: FileFormat = FileFormat {
    id: 105_866_800,
    source_type: SourceType::Wikidata,
    name: "PlayLisT for TSV video",
    extensions: &["plt"],
    media_types: &["video/x-plt"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x55, 0x55, 0x55, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
