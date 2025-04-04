use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859713: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_713,
        source_type: SourceType::Wikidata,
        name: "X86 Delta Compiler Video",
        extensions: &["xdv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x44, 0x43, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
