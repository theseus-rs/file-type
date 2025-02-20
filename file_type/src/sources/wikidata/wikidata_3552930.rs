use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3552930: FileType = FileType {
    file_format: &FileFormat {
        id: 3_552_930,
        source_type: SourceType::Wikidata,
        name: "VGM",
        extensions: &["vgm", "vgz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x67, 0x6D, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
