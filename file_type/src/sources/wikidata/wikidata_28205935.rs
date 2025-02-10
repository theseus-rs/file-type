use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28205935: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_935,
        source_type: SourceType::Wikidata,
        name: "Doodle! compressed image",
        extensions: &["jj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x5C, 0xFE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
