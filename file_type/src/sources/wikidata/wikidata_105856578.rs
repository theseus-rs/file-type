use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856578: FileFormat = FileFormat {
    id: 105_856_578,
    puid: "wikidata/105856578",
    name: "DoomRL WAD resource",
    extensions: &["wad"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x08, 0x56, 0x44, 0x46, 0x49, 0x4C, 0x45, 0x30, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
