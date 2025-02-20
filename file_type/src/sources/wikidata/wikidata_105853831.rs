use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853831: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_831,
        source_type: SourceType::Wikidata,
        name: "Sony OpenMG Audio",
        extensions: &["oma"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x41, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
