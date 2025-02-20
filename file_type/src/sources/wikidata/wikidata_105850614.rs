use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850614: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_614,
        source_type: SourceType::Wikidata,
        name: "Sundial Clearlook document",
        extensions: &["ctx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCE, 0xCE, 0xF6, 0x00, 0x64, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
