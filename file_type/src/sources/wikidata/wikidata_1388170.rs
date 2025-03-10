use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1388170: FileType = FileType {
    file_format: &FileFormat {
        id: 1_388_170,
        source_type: SourceType::Wikidata,
        name: "Value Change Dump",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x24])],
                },
            }],
        }],
        related_formats: &[],
    },
};
