use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2858737: FileType = FileType {
    file_format: &FileFormat {
        id: 2_858_737,
        source_type: SourceType::Wikidata,
        name: "Apple Icon Image format",
        extensions: &["icns"],
        media_types: &["image/x-icns"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x69, 0x63, 0x6E, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
