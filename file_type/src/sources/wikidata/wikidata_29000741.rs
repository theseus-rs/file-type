use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000741: FileFormat = FileFormat {
    id: 29_000_741,
    puid: "wikidata/29000741",
    name: "WorldBuilder",
    extensions: &["wld"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x45, 0x47, 0x49, 0x4E, 0x5F, 0x44, 0x45, 0x53, 0x49, 0x47, 0x4E, 0x3A,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
