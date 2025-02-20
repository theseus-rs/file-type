use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867407: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_407,
        source_type: SourceType::Wikidata,
        name: "CharlieCalc spreadsheet",
        extensions: &["nw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x68, 0x61, 0x72, 0x6C, 0x69, 0x65, 0x43, 0x61, 0x6C, 0x63, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
