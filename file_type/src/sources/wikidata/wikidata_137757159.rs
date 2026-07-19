use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_137757159: FileType = FileType {
    file_format: &FileFormat {
        id: 137_757_159,
        source_type: SourceType::Wikidata,
        name: "Music Director Project",
        extensions: &["mdx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x55, 0x53, 0x49, 0x43, 0x44, 0x49, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
