use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858725: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_725,
        source_type: SourceType::Wikidata,
        name: "BundleFile",
        extensions: &["b"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xBD, 0xF1, 0x42, 0x75, 0x6E, 0x64, 0x6C, 0x65, 0x46, 0x69, 0x6C, 0x65,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
