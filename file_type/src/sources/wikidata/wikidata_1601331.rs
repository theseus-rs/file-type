use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1601331: FileType = FileType {
    file_format: &FileFormat {
        id: 1_601_331,
        source_type: SourceType::Wikidata,
        name: "Transport Neutral Encapsulation Format",
        extensions: &["dat", "tnef"],
        media_types: &["application/vnd.ms-tnef"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x9F, 0x3E, 0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
