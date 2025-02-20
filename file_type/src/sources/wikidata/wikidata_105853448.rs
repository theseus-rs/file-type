use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853448: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_448,
        source_type: SourceType::Wikidata,
        name: "ZEMAX lens data",
        extensions: &["zmx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x45, 0x52, 0x53, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
