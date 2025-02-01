use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864433: FileFormat = FileFormat {
    id: 105_864_433,
    puid: "wikidata/105864433",
    name: "Movie Setter Project",
    extensions: &["prod"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x78, 0x53, 0x63, 0x65, 0x6E, 0x65, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
