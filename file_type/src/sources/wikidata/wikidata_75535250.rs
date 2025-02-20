use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75535250: FileType = FileType {
    file_format: &FileFormat {
        id: 75_535_250,
        source_type: SourceType::Wikidata,
        name: "UFA compressed archive",
        extensions: &["ufa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x46, 0x41, 0xC6, 0xD2, 0xC1])],
                },
            }],
        }],
        related_formats: &[],
    },
};
