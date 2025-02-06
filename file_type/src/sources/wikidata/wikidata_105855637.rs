use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855637: FileFormat = FileFormat {
    id: 105_855_637,
    source_type: SourceType::Wikidata,
    name: "OmniPass encrypted",
    extensions: &["opf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x50, 0x45, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
