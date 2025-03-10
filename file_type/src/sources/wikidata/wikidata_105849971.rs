use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849971: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_971,
        source_type: SourceType::Wikidata,
        name: "BorgBackup Configuration",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x72, 0x65, 0x70, 0x6F, 0x73, 0x69, 0x74, 0x6F, 0x72, 0x79, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
