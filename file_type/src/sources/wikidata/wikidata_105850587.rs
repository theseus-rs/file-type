use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850587: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_587,
        source_type: SourceType::Wikidata,
        name: "PeachCalc spreadsheet (v2.x)",
        extensions: &["cal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x65, 0x61, 0x63, 0x68, 0x43, 0x61, 0x6C, 0x63, 0x20, 0x76, 0x65,
                        0x72, 0x2E, 0x20, 0x20, 0x32, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
