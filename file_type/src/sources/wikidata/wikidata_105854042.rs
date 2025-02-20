use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854042: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_042,
        source_type: SourceType::Wikidata,
        name: "AppleWin SaveState",
        extensions: &["aws"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x57, 0x53, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
