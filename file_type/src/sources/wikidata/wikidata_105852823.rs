use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852823: FileFormat = FileFormat {
    id: 105_852_823,
    puid: "wikidata/105852823",
    name: "Spring Engine 3D model",
    extensions: &["s3o"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x70, 0x72, 0x69, 0x6E, 0x67, 0x20, 0x75, 0x6E, 0x69, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
