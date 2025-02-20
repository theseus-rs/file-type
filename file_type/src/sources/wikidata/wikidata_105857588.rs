use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857588: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_588,
        source_type: SourceType::Wikidata,
        name: "ISAM table handler data",
        extensions: &["ism"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xFE, 0x05, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
