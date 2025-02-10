use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856096: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_096,
        source_type: SourceType::Wikidata,
        name: "Datel Action Replay for Windows 95/98 cheat data",
        extensions: &["dc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x63, 0x31, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
