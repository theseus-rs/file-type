use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862744: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_744,
        source_type: SourceType::Wikidata,
        name: "Turbo Pascal MicroCalc Spreadsheet",
        extensions: &["mcs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x75, 0x72, 0x62, 0x6F, 0x20, 0x50, 0x61, 0x73, 0x63, 0x61, 0x6C,
                        0x20, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x43, 0x61, 0x6C, 0x63, 0x20, 0x53,
                        0x70, 0x72, 0x65, 0x61, 0x64, 0x73, 0x68, 0x65, 0x65, 0x74, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
