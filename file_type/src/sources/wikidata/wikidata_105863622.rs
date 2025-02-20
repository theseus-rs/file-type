use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863622: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_622,
        source_type: SourceType::Wikidata,
        name: "LightWave Motion data",
        extensions: &["mot"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x57, 0x4D, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
