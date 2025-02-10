use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851723: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_723,
        source_type: SourceType::Wikidata,
        name: "Windows Shadow spooler (2003)",
        extensions: &["shd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x49, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
