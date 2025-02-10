use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858612: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_612,
        source_type: SourceType::Wikidata,
        name: "CASIO WinCE PDA backup",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x53, 0x49, 0x4F, 0x20, 0x57, 0x69, 0x6E, 0x43, 0x45, 0x20,
                        0x50, 0x44, 0x41, 0x20, 0x62, 0x61, 0x63, 0x6B, 0x75, 0x70, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x20, 0x61, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
