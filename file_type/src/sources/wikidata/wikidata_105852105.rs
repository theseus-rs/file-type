use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852105: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_105,
        source_type: SourceType::Wikidata,
        name: "StoneCracker S401 compressed",
        extensions: &["stc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x34, 0x30, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
