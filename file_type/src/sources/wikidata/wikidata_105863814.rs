use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_814,
        source_type: SourceType::Wikidata,
        name: "MinGW Developer Studio Project",
        extensions: &["mdsp"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
