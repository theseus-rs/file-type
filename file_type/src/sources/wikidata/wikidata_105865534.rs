use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865534: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_534,
        source_type: SourceType::Wikidata,
        name: "Personal Font Maker Preferences",
        extensions: &["prf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x46, 0x4D, 0x20, 0x50, 0x52, 0x45, 0x46, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
