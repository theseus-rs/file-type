use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858478: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_478,
        source_type: SourceType::Wikidata,
        name: "CODESYS ECI data",
        extensions: &["eci"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6F, 0x44, 0x65, 0x53, 0x79, 0x73, 0x20, 0x45, 0x43, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
