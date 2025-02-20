use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852142: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_142,
        source_type: SourceType::Wikidata,
        name: "Quartus Symbols data",
        extensions: &["sym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x59, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
