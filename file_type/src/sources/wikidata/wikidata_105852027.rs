use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852027: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_027,
        source_type: SourceType::Wikidata,
        name: "DDP Image Stream Descriptor",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x56, 0x56, 0x53, 0x30, 0x30, 0x30, 0x30, 0x20, 0x20, 0x30, 0x30,
                        0x30, 0x30, 0x30, 0x30, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
