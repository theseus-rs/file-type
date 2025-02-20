use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855890: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_890,
        source_type: SourceType::Wikidata,
        name: "CopyQM disk image",
        extensions: &["cpy", "cqm", "dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x51, 0x14])],
                },
            }],
        }],
        related_formats: &[],
    },
};
