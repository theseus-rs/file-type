use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863666: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_666,
        source_type: SourceType::Wikidata,
        name: "Project: Space Station saved Mission",
        extensions: &["msn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x80, 0x50, 0x80, 0x4D, 0x80, 0x4B, 0x80, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
