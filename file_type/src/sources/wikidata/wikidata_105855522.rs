use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855522: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_522,
        source_type: SourceType::Wikidata,
        name: "dBASE compiled Format",
        extensions: &["fmo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x05, 0x44, 0x42, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
