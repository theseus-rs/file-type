use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857990: FileFormat = FileFormat {
    id: 105_857_990,
    puid: "wikidata/105857990",
    name: "DOSIMG disk image (80t/18s)",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x12, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
