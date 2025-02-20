use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865275: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_275,
        source_type: SourceType::Wikidata,
        name: "Process Monitor Log (native format)",
        extensions: &["pml"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x4C, 0x5F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
