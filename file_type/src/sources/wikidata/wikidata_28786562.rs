use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28786562: FileType = FileType {
    file_format: &FileFormat {
        id: 28_786_562,
        source_type: SourceType::Wikidata,
        name: "Nickfile",
        extensions: &["nk2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0xF0, 0xAD, 0xBA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
