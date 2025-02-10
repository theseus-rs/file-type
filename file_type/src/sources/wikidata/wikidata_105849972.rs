use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849972: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_972,
        source_type: SourceType::Wikidata,
        name: "COMX-35 program",
        extensions: &["comx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x4D, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
