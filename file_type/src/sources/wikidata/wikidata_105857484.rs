use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857484: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_484,
        source_type: SourceType::Wikidata,
        name: "SCUMM main data container (v6)",
        extensions: &["001"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x45, 0x43, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
