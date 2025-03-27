use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1228757: FileType = FileType {
    file_format: &FileFormat {
        id: 1_228_757,
        source_type: SourceType::Wikidata,
        name: "Apple Disk Image",
        extensions: &["dmg", "image", "img", "smi"],
        media_types: &["application/x-apple-diskimage"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x01, 0x73, 0x0D, 0x62, 0x62, 0x60])],
                },
            }],
        }],
        related_formats: &[],
    },
};
