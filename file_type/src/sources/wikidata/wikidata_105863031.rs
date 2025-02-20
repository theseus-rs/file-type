use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863031: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_031,
        source_type: SourceType::Wikidata,
        name: "h8 tracker module",
        extensions: &["h8s"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x38, 0x74, 0x72, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
