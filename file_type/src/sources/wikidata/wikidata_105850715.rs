use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850715: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_715,
        source_type: SourceType::Wikidata,
        name: "Kalles Fraktaler parameters",
        extensions: &["kfr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x65, 0x3A, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
