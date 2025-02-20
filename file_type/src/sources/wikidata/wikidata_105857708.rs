use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857708: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_708,
        source_type: SourceType::Wikidata,
        name: "F64 disk image",
        extensions: &["f64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
