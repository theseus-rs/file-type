use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857410: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_410,
        source_type: SourceType::Wikidata,
        name: "EditPad Pro Custom Syntax Coloring Scheme (UTF-8)",
        extensions: &["jgcscs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x4A, 0x47, 0x43, 0x53, 0x43, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
