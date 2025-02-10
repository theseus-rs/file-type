use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863329: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_329,
        source_type: SourceType::Wikidata,
        name: "Cubic Tiny XM module",
        extensions: &["mxm"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x58, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
