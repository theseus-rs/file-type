use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857743: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_743,
        source_type: SourceType::Wikidata,
        name: "Ensoniq EPS EDM disk image",
        extensions: &["ede"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x0A, 0x45, 0x50, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
