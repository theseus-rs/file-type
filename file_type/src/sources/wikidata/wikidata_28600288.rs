use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28600288: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_288,
        source_type: SourceType::Wikidata,
        name: "ActiveMime",
        extensions: &["mso"],
        media_types: &["application/x-mso"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x4D, 0x69, 0x6D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
