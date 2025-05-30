use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71832451: FileType = FileType {
    file_format: &FileFormat {
        id: 71_832_451,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 1",
        extensions: &["cdr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x4C, 0x65, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
