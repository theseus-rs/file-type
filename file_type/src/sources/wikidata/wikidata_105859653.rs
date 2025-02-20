use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859653: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_653,
        source_type: SourceType::Wikidata,
        name: "Reaper video",
        extensions: &["fmv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x52, 0x65, 0x61, 0x70, 0x65, 0x72, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
