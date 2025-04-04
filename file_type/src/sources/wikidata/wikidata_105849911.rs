use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849911: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_911,
        source_type: SourceType::Wikidata,
        name: "WinWorks Chart",
        extensions: &["cht"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x69, 0x6E, 0x57, 0x6F, 0x72, 0x6B, 0x73, 0x20, 0x43, 0x68, 0x61,
                        0x72, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
