use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855473: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_473,
        source_type: SourceType::Wikidata,
        name: "FL Studio Track",
        extensions: &["fst"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4C, 0x68, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
