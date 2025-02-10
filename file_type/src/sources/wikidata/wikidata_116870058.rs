use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_116870058: FileType = FileType {
    file_format: &FileFormat {
        id: 116_870_058,
        source_type: SourceType::Wikidata,
        name: "MOOF Disk Image",
        extensions: &["moof"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x4F, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
