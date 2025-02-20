use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853087: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_087,
        source_type: SourceType::Wikidata,
        name: "Siag spreadsheet",
        extensions: &["siag"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x73, 0x77, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
