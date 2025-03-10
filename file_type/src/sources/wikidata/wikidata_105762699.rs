use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762699: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_699,
        source_type: SourceType::Wikidata,
        name: "Binary Device Interface File Format",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x49, 0x46, 0x42, 0x49, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
