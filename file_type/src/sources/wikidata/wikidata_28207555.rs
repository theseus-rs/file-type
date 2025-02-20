use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207555: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_555,
        source_type: SourceType::Wikidata,
        name: "eXtended Image File Format, version 3",
        extensions: &["xif"],
        media_types: &["image/vnd.xiff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x2A, 0x00, 0x5C, 0x01, 0x00, 0x00, 0x20, 0x65, 0x58, 0x74,
                        0x65, 0x6E, 0x64, 0x65, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
