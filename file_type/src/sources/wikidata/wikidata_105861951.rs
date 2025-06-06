use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861951: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_951,
        source_type: SourceType::Wikidata,
        name: "MATLAB Mac 64bit compiled function",
        extensions: &["mexmaci64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCF, 0xFA, 0xED, 0xFE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
