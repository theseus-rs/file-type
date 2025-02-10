use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_71831258: FileType = FileType {
    file_format: &FileFormat {
        id: 71_831_258,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 2",
        extensions: &["cdr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x4C, 0x6D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
