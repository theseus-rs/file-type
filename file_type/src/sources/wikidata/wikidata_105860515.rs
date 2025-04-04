use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860515: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_515,
        source_type: SourceType::Wikidata,
        name: "Windows Registry Data",
        extensions: &["reg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x45, 0x47, 0x45, 0x44, 0x49, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
