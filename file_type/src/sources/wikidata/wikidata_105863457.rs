use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863457: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_457,
        source_type: SourceType::Wikidata,
        name: "Microsoft Baseline Security Analyser report",
        extensions: &["mbsa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x3C, 0x00, 0x53, 0x00, 0x65, 0x00, 0x63, 0x00, 0x53, 0x00,
                        0x63, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x20, 0x00, 0x49, 0x00, 0x44, 0x00,
                        0x3D, 0x00, 0x22, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
