use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866800: FileFormat = FileFormat {
    id: 105_866_800,
    puid: "wikidata/105866800",
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
