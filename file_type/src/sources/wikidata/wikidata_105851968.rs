use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851968: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_968,
        source_type: SourceType::Wikidata,
        name: "SAdT music composer module/song",
        extensions: &["sat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x41, 0x64, 0x54, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
