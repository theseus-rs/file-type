use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853974: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_974,
        source_type: SourceType::Wikidata,
        name: "AutoREALM Symbols",
        extensions: &["aus"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x75, 0x74, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
