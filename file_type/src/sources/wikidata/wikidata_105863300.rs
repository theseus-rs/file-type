use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863300: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_300,
        source_type: SourceType::Wikidata,
        name: "SMS Material",
        extensions: &["mat", "materials"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x41, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
