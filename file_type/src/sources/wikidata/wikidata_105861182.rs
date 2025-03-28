use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861182: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_182,
        source_type: SourceType::Wikidata,
        name: "Siemens ORSI Log",
        extensions: &["log"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x52, 0x53, 0x49, 0x4C, 0x4F, 0x47, 0x46, 0x49, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
