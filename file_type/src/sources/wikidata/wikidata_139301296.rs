use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_139301296: FileType = FileType {
    file_format: &FileFormat {
        id: 139_301_296,
        source_type: SourceType::Wikidata,
        name: "Now Contact File",
        extensions: &["nct"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xBE, 0xDE, 0xAD, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
