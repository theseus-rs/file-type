use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_931783: FileType = FileType {
    file_format: &FileFormat {
        id: 931_783,
        source_type: SourceType::Wikidata,
        name: "JPEG 2000",
        extensions: &[
            "j2c", "j2k", "jp2", "jpc", "jpf", "jpg2", "jpm", "jpx", "mj2",
        ],
        media_types: &["image/jp2", "image/jpm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x0C, 0x6A, 0x50, 0x20, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
