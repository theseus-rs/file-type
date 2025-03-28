use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855732: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_732,
        source_type: SourceType::Wikidata,
        name: "Deep Thought disk Image",
        extensions: &["dti"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x32, 0x47, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
