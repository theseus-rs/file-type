use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863128: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_128,
        source_type: SourceType::Wikidata,
        name: "MediaView document",
        extensions: &["mdv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x25, 0x6E, 0x78, 0x62, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
