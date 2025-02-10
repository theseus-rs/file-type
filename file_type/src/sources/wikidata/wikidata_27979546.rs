use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979546: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_546,
        source_type: SourceType::Wikidata,
        name: "Civilization III saved game format",
        extensions: &["bic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x49, 0x43, 0x20, 0x56, 0x45, 0x52, 0x23,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
