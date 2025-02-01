use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859557: FileFormat = FileFormat {
    id: 105_859_557,
    puid: "wikidata/105859557",
    name: "SMJPEG Video",
    extensions: &["mjpg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5E, 0x40, 0x5C, 0x6E, 0x53, 0x4D, 0x4A, 0x50, 0x45, 0x47,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
