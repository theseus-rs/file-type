use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853249: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_249,
        source_type: SourceType::Wikidata,
        name: "Live for Speed data",
        extensions: &["spr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x46, 0x53, 0x53, 0x50, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
