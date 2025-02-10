use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852223: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_223,
        source_type: SourceType::Wikidata,
        name: "Altera Synopsys Constraints Format",
        extensions: &["scf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x43, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
