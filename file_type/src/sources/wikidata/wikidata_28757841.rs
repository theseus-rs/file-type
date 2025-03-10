use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757841: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_841,
        source_type: SourceType::Wikidata,
        name: "Genecyst Save State",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x53, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
