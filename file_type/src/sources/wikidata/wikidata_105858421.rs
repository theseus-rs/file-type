use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858421: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_421,
        source_type: SourceType::Wikidata,
        name: "Preferred Executable Format (68K)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x6F, 0x79, 0x21, 0x70, 0x65, 0x66, 0x66, 0x6D, 0x36, 0x38, 0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
