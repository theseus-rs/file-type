use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979394: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_394,
        source_type: SourceType::Wikidata,
        name: "DVM",
        extensions: &["dvm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x56, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
