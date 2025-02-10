use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866450: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_450,
        source_type: SourceType::Wikidata,
        name: "Commodore PET BASIC 4.0 program",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
