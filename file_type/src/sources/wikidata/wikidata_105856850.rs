use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856850: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_850,
        source_type: SourceType::Wikidata,
        name: "Indiana Jones and the Infernal Machine Game data archive",
        extensions: &["gob"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4F, 0x42, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
