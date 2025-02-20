use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858126: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_126,
        source_type: SourceType::Wikidata,
        name: "Diablo 1 Item safe file format",
        extensions: &["itm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x54, 0x4D, 0x30, 0x31, 0x2E, 0x49, 0x27, 0x6C, 0x6C, 0x20, 0x67,
                        0x65, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6C, 0x27, 0x54,
                        0x68, 0x6F, 0x72, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
