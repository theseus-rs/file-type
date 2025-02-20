use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855949: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_949,
        source_type: SourceType::Wikidata,
        name: "DMIS input data",
        extensions: &["dmi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
