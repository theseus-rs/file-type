use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849968: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_968,
        source_type: SourceType::Wikidata,
        name: "Turbo Pascal 2.0 Chain module",
        extensions: &["chn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE8, 0x03, 0xE3])],
                },
            }],
        }],
        related_formats: &[],
    },
};
