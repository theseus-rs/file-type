use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_287,
        source_type: SourceType::Wikidata,
        name: "Error Code Modeler",
        extensions: &["ecm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x43, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
