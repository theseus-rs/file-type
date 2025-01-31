use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850661: FileFormat = FileFormat {
    id: 105_850_661,
    puid: "wikidata/105850661",
    name: "Voxlap voxel sprite",
    extensions: &["kv6"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x76, 0x78, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
