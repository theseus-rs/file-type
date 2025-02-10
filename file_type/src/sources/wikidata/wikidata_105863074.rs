use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863074: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_074,
        source_type: SourceType::Wikidata,
        name: "VGM Music Maker module",
        extensions: &["vge"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x47, 0x45, 0x66, 0x6D, 0x74, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
