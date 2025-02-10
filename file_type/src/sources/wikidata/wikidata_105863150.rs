use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863150: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_150,
        source_type: SourceType::Wikidata,
        name: "Mark II Soundsystem song",
        extensions: &["mii"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x41, 0x44, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
