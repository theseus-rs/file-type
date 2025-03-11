use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850522: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_522,
        source_type: SourceType::Wikidata,
        name: "CygnusEd default settings",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x45, 0x66, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
