use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857452: FileFormat = FileFormat {
    id: 105_857_452,
    puid: "wikidata/105857452",
    name: "Shaper LUT",
    extensions: &["3dl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x4D, 0x45, 0x53, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
