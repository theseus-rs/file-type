use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857846: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_846,
        source_type: SourceType::Wikidata,
        name: "OpenSceneGraph native binary format",
        extensions: &["ive"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x03, 0x02, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
