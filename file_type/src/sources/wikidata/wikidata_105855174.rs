use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855174: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_174,
        source_type: SourceType::Wikidata,
        name: "WinMerge directory/file Filter",
        extensions: &["flt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x23, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61,
                        0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x79, 0x2F, 0x66,
                        0x69, 0x6C, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x74, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
