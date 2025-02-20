use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867627: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_627,
        source_type: SourceType::Wikidata,
        name: "NTitler show",
        extensions: &["nt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x54, 0x39, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
