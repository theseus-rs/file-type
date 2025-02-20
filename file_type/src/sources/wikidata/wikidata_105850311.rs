use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850311: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_311,
        source_type: SourceType::Wikidata,
        name: "Javelin Case study",
        extensions: &["cas"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF7, 0x12, 0x34, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
