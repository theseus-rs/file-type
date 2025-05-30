use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861935: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_935,
        source_type: SourceType::Wikidata,
        name: "Tecplot Macro",
        extensions: &["mcr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x21, 0x4D, 0x43, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
