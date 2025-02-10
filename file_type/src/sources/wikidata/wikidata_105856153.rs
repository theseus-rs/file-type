use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856153: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_153,
        source_type: SourceType::Wikidata,
        name: "Macintosh Disk image (BZlib compressed)",
        extensions: &["dmg"],
        media_types: &["application/x-apple-diskimage"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
