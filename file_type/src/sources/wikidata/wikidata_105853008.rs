use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853008: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_008,
        source_type: SourceType::Wikidata,
        name: "Symbian Series 3 Installation file",
        extensions: &["sisx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7A, 0x1A, 0x20, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
