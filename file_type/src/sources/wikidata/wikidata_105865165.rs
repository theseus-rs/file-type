use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865165: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_165,
        source_type: SourceType::Wikidata,
        name: "Mac OS X Binary-format PList",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
