use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866775: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_775,
        source_type: SourceType::Wikidata,
        name: "Adorage preferences",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x04, 0x46, 0x45, 0x45, 0x70, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
