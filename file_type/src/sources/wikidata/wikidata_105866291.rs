use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866291: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_291,
        source_type: SourceType::Wikidata,
        name: "Polyfilm Preferences",
        extensions: &["prf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x6F, 0x6C, 0x79, 0x5F, 0x50, 0x72, 0x66,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
