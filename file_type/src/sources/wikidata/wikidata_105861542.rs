use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861542: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_542,
        source_type: SourceType::Wikidata,
        name: "SimTex LBX game data container",
        extensions: &["lbx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAD, 0xFE, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
