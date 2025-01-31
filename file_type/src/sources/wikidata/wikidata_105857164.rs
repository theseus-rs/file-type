use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857164: FileFormat = FileFormat {
    id: 105_857_164,
    puid: "wikidata/105857164",
    name: "Microsoft Hearts Saved game",
    extensions: &["heartssave-ms"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x47, 0x4D, 0x48, 0x01, 0x00, 0x00, 0x00, 0x28, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
