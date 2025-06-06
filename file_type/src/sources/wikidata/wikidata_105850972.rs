use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850972: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_972,
        source_type: SourceType::Wikidata,
        name: "DB/TextWorks Database Primary Textbase Definition",
        extensions: &["tba"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x42, 0x41, 0x20, 0x30, 0x32, 0x30, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
