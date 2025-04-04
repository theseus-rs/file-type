use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852956: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_956,
        source_type: SourceType::Wikidata,
        name: "Lotus ScreenCam Caption Script",
        extensions: &["scs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x21, 0x00, 0x00, 0x00, 0x2E, 0x4C, 0x6F, 0x74, 0x75, 0x73,
                        0x2E, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x43, 0x61, 0x6D, 0x2E, 0x43,
                        0x61, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x2E, 0x53, 0x63, 0x72, 0x69, 0x70,
                        0x74, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
