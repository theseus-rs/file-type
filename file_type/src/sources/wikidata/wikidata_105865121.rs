use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865121: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_121,
        source_type: SourceType::Wikidata,
        name: "World Construction Set Preferences",
        extensions: &["prefs"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x43, 0x53, 0x4D, 0x56, 0x50, 0x72, 0x65, 0x66, 0x73, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
