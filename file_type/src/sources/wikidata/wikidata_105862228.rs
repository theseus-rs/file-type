use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862228: FileFormat = FileFormat {
    id: 105_862_228,
    source_type: SourceType::Wikidata,
    name: "MapDraw chemical sequence map",
    extensions: &["mpd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x65, 0x00, 0x00, 0x00, 0x10, 0x00,
                    0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
