use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866693: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_693,
        source_type: SourceType::Wikidata,
        name: "Sigfried Antivirus Professional Preferences (v1.0)",
        extensions: &["prefs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x46, 0x50, 0x56, 0x31, 0x2E, 0x30, 0x00, 0x53, 0x46, 0x41, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
