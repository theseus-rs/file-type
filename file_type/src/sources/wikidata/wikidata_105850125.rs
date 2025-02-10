use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850125: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_125,
        source_type: SourceType::Wikidata,
        name: "Ease Calcform spreadsheet",
        extensions: &["cal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x50, 0x45, 0x52, 0x41, 0x1A, 0x0A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
