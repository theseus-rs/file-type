use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852795: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_795,
        source_type: SourceType::Wikidata,
        name: "AutoCAD STereoLithography (binary)",
        extensions: &["stl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x73, 0x6F, 0x6C, 0x69,
                        0x64, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
