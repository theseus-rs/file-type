use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854984: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_984,
        source_type: SourceType::Wikidata,
        name: "ADX2 HCA Audio",
        extensions: &["awb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x46, 0x53, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
