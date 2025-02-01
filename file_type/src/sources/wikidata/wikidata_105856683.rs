use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856683: FileFormat = FileFormat {
    id: 105_856_683,
    puid: "wikidata/105856683",
    name: "UTAU vocal track",
    extensions: &["ust"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x23, 0x53, 0x45, 0x54, 0x54, 0x49, 0x4E, 0x47, 0x5D, 0x0D, 0x0A, 0x54,
                    0x65, 0x6D, 0x70, 0x6F, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
