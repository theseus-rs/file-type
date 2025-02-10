use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863026: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_026,
        source_type: SourceType::Wikidata,
        name: "DCAlice snapshot",
        extensions: &["mrx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x06, 0x33, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
