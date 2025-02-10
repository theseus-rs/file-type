use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855711: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_711,
        source_type: SourceType::Wikidata,
        name: "Open Scenegraph scene",
        extensions: &["osg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x72, 0x6F, 0x75, 0x70, 0x20, 0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
