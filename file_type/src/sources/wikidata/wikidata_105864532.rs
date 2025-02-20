use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864532: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_532,
        source_type: SourceType::Wikidata,
        name: "Brown Bag Word Processor Printer control ruler",
        extensions: &["prt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x50, 0x72, 0x69, 0x6E, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6F, 0x6E,
                        0x74, 0x72, 0x6F, 0x6C, 0x20, 0x72, 0x75, 0x6C, 0x65, 0x72, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x29,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
