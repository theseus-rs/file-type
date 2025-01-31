use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866145: FileFormat = FileFormat {
    id: 105_866_145,
    puid: "wikidata/105866145",
    name: "PxTone Collage module (protected)",
    extensions: &["pttune"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x54, 0x54, 0x55, 0x4E, 0x45, 0x2D, 0x2D, 0x32, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
