use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853185: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_185,
        source_type: SourceType::Wikidata,
        name: "SecurID Soft Token",
        extensions: &["sdtid"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
