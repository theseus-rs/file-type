use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770328: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_328,
        source_type: SourceType::Wikidata,
        name: "Libwww-perl cookie jar",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x4C, 0x57, 0x50, 0x2D, 0x43, 0x6F, 0x6F, 0x6B, 0x69, 0x65, 0x73,
                        0x2D, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
