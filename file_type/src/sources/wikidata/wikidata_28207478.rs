use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207478: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_478,
        source_type: SourceType::Wikidata,
        name: "Webshots picture WBC",
        extensions: &["wbc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAB, 0x16, 0xFA, 0x95])],
                },
            }],
        }],
        related_formats: &[],
    },
};
