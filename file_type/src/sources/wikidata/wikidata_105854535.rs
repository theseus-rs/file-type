use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854535: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_535,
        source_type: SourceType::Wikidata,
        name: "WinHki compressed archive",
        extensions: &["hki"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x5C, 0x04, 0x05, 0x14, 0x41, 0x00, 0x00, 0xFD, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
