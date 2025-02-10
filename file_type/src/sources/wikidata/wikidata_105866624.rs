use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866624: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_624,
        source_type: SourceType::Wikidata,
        name: "Novalogic game data archive (PFF0)",
        extensions: &["pff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x46, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
