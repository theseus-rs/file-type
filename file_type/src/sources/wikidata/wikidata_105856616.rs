use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856616: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_616,
        source_type: SourceType::Wikidata,
        name: "World of Warcraft WDC1 database",
        extensions: &["db2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x44, 0x43, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
