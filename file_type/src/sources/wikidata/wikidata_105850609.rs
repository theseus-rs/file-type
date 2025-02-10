use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850609: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_609,
        source_type: SourceType::Wikidata,
        name: "Lotus Symphony configuration",
        extensions: &["cnf"],
        media_types: &["application/vnd.lotus-1-2-3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x02, 0x08])],
                },
            }],
        }],
        related_formats: &[],
    },
};
