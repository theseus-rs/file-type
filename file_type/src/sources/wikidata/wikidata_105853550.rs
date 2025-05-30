use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853550: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_550,
        source_type: SourceType::Wikidata,
        name: "ZDA game data archive",
        extensions: &["zda"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x44, 0x41, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
