use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_33517407: FileFormat = FileFormat {
    id: 33_517_407,
    puid: "wikidata/33517407",
    name: "E57 Lidar Point Cloud Format",
    extensions: &["e57"],
    media_types: &["model/e57"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x53, 0x54, 0x4D, 0x2D, 0x45, 0x35, 0x37,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
