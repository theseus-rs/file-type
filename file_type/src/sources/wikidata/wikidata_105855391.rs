use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855391: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_391,
        source_type: SourceType::Wikidata,
        name: "RIPterm Font",
        extensions: &["fnt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x04, 0x20, 0x52, 0x49, 0x50, 0x74, 0x65, 0x72, 0x6D, 0x20, 0x46, 0x6F,
                        0x6E, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
