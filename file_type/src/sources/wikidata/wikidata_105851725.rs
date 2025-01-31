use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851725: FileFormat = FileFormat {
    id: 105_851_725,
    puid: "wikidata/105851725",
    name: "Superbase Project",
    extensions: &["sbj"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x49, 0x5D, 0x0D, 0x0A, 0x4F, 0x70, 0x65, 0x6E, 0x41, 0x75, 0x74,
                    0x6F, 0x41, 0x64, 0x64, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
