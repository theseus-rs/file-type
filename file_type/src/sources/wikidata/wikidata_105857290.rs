use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857290: FileFormat = FileFormat {
    id: 105_857_290,
    puid: "wikidata/105857290",
    name: "Human Machine Interfaces MIDI Format (rev.2)",
    extensions: &["hmi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x4D, 0x49, 0x2D, 0x4D, 0x49, 0x44, 0x49, 0x53, 0x4F, 0x4E, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
