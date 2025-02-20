use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853723: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_723,
        source_type: SourceType::Wikidata,
        name: "Chasm BIN archive",
        extensions: &["bin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x53, 0x69, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
