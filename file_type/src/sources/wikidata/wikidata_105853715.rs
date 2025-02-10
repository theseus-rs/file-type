use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853715: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_715,
        source_type: SourceType::Wikidata,
        name: "Liquid Entertainment H2O game data archive",
        extensions: &["h2o"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x49, 0x51, 0x44, 0x4C, 0x48, 0x32, 0x4F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
