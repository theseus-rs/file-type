use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762671: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_671,
        source_type: SourceType::Wikidata,
        name: "XACT Sound Bank",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x44, 0x42, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
