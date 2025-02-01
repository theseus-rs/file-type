use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858489: FileFormat = FileFormat {
    id: 105_858_489,
    puid: "wikidata/105858489",
    name: "APP raster bitmap",
    extensions: &["app"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x31, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
