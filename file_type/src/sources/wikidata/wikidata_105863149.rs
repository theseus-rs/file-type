use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863149: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_149,
        source_type: SourceType::Wikidata,
        name: "Multimedia Viewer Book",
        extensions: &["mvb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3F, 0x5F, 0x03, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
