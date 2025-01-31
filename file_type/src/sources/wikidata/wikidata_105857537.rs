use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857537: FileFormat = FileFormat {
    id: 105_857_537,
    puid: "wikidata/105857537",
    name: "LDBS disk image (v0.3)",
    extensions: &["ldbs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x42, 0x53, 0x01, 0x44, 0x53, 0x4B, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
