use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853676: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_676,
        source_type: SourceType::Wikidata,
        name: "AOS File Format",
        extensions: &["aos"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x49, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
