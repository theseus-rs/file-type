use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2192: FileType = FileType {
    file_format: &FileFormat {
        id: 2_192,
        source_type: SourceType::Wikidata,
        name: "GIF",
        extensions: &["gif"],
        media_types: &["image/gif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x49, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
