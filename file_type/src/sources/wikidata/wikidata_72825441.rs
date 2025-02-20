use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72825441: FileType = FileType {
    file_format: &FileFormat {
        id: 72_825_441,
        source_type: SourceType::Wikidata,
        name: "Psion Organiser Block data",
        extensions: &["obx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x52, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
