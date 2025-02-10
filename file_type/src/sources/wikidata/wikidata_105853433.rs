use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853433: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_433,
        source_type: SourceType::Wikidata,
        name: "Zeno format",
        extensions: &["zeno"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA3, 0xA0, 0xD2, 0x55])],
                },
            }],
        }],
        related_formats: &[],
    },
};
