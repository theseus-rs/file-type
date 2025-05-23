use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850595: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_595,
        source_type: SourceType::Wikidata,
        name: "Cardwar Cards deck",
        extensions: &["cwd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
