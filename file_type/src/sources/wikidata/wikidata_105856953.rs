use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856953: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_953,
        source_type: SourceType::Wikidata,
        name: "Farming Simulator terrein data",
        extensions: &["grle"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x52, 0x4C, 0x45, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
