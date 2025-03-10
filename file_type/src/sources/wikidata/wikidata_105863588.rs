use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863588: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_588,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works for Mac document (v1.0)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
