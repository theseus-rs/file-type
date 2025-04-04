use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861394: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_394,
        source_type: SourceType::Wikidata,
        name: "MATLAB license passcode",
        extensions: &["lic"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x4D, 0x41, 0x54, 0x4C, 0x41, 0x42, 0x20, 0x6C, 0x69, 0x63,
                        0x65, 0x6E, 0x73, 0x65, 0x20, 0x70, 0x61, 0x73, 0x73, 0x63, 0x6F, 0x64,
                        0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
