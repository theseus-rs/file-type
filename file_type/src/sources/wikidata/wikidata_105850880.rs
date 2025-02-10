use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850880: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_880,
        source_type: SourceType::Wikidata,
        name: "3D Construction Kit 2 World Data (Amiga)",
        extensions: &["kwd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x6D, 0x69, 0x67, 0x61, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                        0x20, 0x33, 0x44, 0x20, 0x4B, 0x69, 0x74, 0x20, 0x32, 0x20, 0x44, 0x61,
                        0x74, 0x61, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
