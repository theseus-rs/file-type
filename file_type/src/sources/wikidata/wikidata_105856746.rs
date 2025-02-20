use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856746: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_746,
        source_type: SourceType::Wikidata,
        name: "Unreal Engine Plugin",
        extensions: &["uplugin"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
