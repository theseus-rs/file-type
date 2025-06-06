use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863303: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_303,
        source_type: SourceType::Wikidata,
        name: "SawTeeth module",
        extensions: &["st"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x57, 0x54, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
