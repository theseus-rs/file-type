use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851790: FileFormat = FileFormat {
    id: 105_851_790,
    puid: "wikidata/105851790",
    name: "Wingz script",
    extensions: &["scz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x4E, 0x47, 0x5A, 0x57, 0x5A, 0x53, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
