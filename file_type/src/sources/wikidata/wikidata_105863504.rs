use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863504: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_504,
        source_type: SourceType::Wikidata,
        name: "Hi-MD Minidisc DRM keyfile",
        extensions: &["hma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x4C, 0x53, 0x54, 0x00, 0x00, 0x01, 0x25, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
