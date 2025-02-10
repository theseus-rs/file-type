use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857151: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_151,
        source_type: SourceType::Wikidata,
        name: "Total Annihilation saved game",
        extensions: &["hpi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x41, 0x50, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
