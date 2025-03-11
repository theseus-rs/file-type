use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_814,
        source_type: SourceType::Wikidata,
        name: "Application Techniques Pizazz compressed",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x70, 0x5A, 0x7A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
