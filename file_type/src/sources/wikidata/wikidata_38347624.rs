use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_38347624: FileType = FileType {
    file_format: &FileFormat {
        id: 38_347_624,
        source_type: SourceType::Wikidata,
        name: "numpy",
        extensions: &["npy"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x93, 0x4E, 0x55, 0x4D, 0x50, 0x59])],
                },
            }],
        }],
        related_formats: &[],
    },
};
