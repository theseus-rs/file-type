use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852890: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_890,
        source_type: SourceType::Wikidata,
        name: "JS Mocha Snapshot",
        extensions: &["snp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x37, 0x50, 0x39, 0x4F, 0x64, 0x6F, 0x78, 0x79, 0x44, 0x6A,
                        0x37, 0x7A, 0x63, 0x56, 0x44, 0x32, 0x43, 0x42, 0x42, 0x32, 0x38, 0x56,
                        0x39, 0x31, 0x67, 0x42, 0x42, 0x31, 0x44, 0x71, 0x4B, 0x75, 0x54, 0x72,
                        0x37, 0x33, 0x67, 0x75, 0x63, 0x47, 0x63, 0x38, 0x64, 0x61, 0x47, 0x63,
                        0x61, 0x38, 0x63,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
