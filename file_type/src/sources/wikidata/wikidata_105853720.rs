use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853720: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_720,
        source_type: SourceType::Wikidata,
        name: "Abacus spreadsheet",
        extensions: &["aba"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x42, 0x4D, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
