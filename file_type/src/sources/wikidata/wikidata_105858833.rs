use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858833: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_833,
        source_type: SourceType::Wikidata,
        name: "Signum! bitmap",
        extensions: &["imc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x62, 0x69, 0x6D, 0x63, 0x30, 0x30, 0x30, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
