use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863457: FileFormat = FileFormat {
    id: 105_863_457,
    puid: "wikidata/105863457",
    name: "Microsoft Baseline Security Analyser report",
    extensions: &["mbsa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x3C, 0x00, 0x53, 0x00, 0x65, 0x00, 0x63, 0x00, 0x53, 0x00, 0x63,
                    0x00, 0x61, 0x00, 0x6E, 0x00, 0x20, 0x00, 0x49, 0x00, 0x44, 0x00, 0x3D, 0x00,
                    0x22, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
