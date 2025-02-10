use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856732: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_732,
        source_type: SourceType::Wikidata,
        name: "VisualBoyAdvance UPS patch",
        extensions: &["ups"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x50, 0x53, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
