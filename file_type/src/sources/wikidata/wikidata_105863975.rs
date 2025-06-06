use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863975: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_975,
        source_type: SourceType::Wikidata,
        name: "Mass Effect save game",
        extensions: &["masseffectsave"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x47, 0x4D, 0x48, 0x01, 0x00, 0x00, 0x00, 0x28, 0x20, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x86, 0x74, 0x03, 0xD7, 0x86, 0x72, 0x17, 0x4D, 0x8E, 0xCE, 0xAE, 0xFD,
                        0x2E, 0x0F, 0x46, 0xB2, 0x4D, 0x00, 0x61, 0x00, 0x73, 0x00, 0x73, 0x00,
                        0x45, 0x00, 0x66, 0x00, 0x66, 0x00, 0x65, 0x00, 0x63, 0x00, 0x74, 0x00,
                        0x2E, 0x00, 0x65, 0x00, 0x78, 0x00, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
