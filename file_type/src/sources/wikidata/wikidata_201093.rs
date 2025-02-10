use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_201093: FileType = FileType {
    file_format: &FileFormat {
        id: 201_093,
        source_type: SourceType::Wikidata,
        name: "RealAudio",
        extensions: &["ra", "ram"],
        media_types: &[
            "audio/vnd.rn-realaudio",
            "audio/x-pn-realaudio",
            "audio/x-realaudio",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2E, 0x52, 0x4D, 0x46, 0x00, 0x00, 0x00, 0x12, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
