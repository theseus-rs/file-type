use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852619: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_619,
        source_type: SourceType::Wikidata,
        name: "Alpha Four Search List",
        extensions: &["sln"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x34, 0x0E, 0x0A, 0x00, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x20,
                        0x4C, 0x69, 0x73, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
