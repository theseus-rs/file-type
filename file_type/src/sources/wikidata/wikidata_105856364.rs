use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856364: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_364,
        source_type: SourceType::Wikidata,
        name: "Macintosh encrypted Disk image (v2)",
        extensions: &["dmg"],
        media_types: &["application/x-apple-diskimage"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x65, 0x6E, 0x63, 0x72, 0x63, 0x64, 0x73, 0x61,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
