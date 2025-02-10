use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863647: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_647,
        source_type: SourceType::Wikidata,
        name: "MyLifeOrganized data",
        extensions: &["ml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x79, 0x4C, 0x69, 0x66, 0x65, 0x44, 0x61, 0x74, 0x61, 0x46, 0x69,
                        0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
