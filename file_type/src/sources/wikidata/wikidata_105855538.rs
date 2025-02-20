use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855538: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_538,
        source_type: SourceType::Wikidata,
        name: "OziExplorer Map",
        extensions: &["ozfx3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x77])],
                },
            }],
        }],
        related_formats: &[],
    },
};
