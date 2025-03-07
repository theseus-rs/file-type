use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60767426: FileType = FileType {
    file_format: &FileFormat {
        id: 60_767_426,
        source_type: SourceType::Wikidata,
        name: "Apache ORC",
        extensions: &["orc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x52, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
