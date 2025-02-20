use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757949: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_949,
        source_type: SourceType::Wikidata,
        name: "HA",
        extensions: &["ha"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
