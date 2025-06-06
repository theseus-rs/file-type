use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851385: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_385,
        source_type: SourceType::Wikidata,
        name: "WinTVC Snapshot",
        extensions: &["tvs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x53, 0x30, 0x31, 0x43, 0x50, 0x55, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
