use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863145: FileFormat = FileFormat {
    id: 105_863_145,
    puid: "wikidata/105863145",
    name: "MeshLab filter script",
    extensions: &["mlx"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x46, 0x69, 0x6C,
                    0x74, 0x65, 0x72, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x3E, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
