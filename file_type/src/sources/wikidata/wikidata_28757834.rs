use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757834: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_834,
        source_type: SourceType::Wikidata,
        name: "Garry's Mod Addon",
        extensions: &["gma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4D, 0x41, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
