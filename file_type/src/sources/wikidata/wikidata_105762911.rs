use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762911: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_911,
        source_type: SourceType::Wikidata,
        name: "MagicaVoxel palette",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x58, 0x52, 0x41, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
