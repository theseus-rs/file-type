use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852112: FileFormat = FileFormat {
    id: 105_852_112,
    puid: "wikidata/105852112",
    name: "Sound Club module",
    extensions: &["sn"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4E, 0x47, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
