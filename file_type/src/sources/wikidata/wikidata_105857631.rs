use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857631: FileFormat = FileFormat {
    id: 105_857_631,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine tileset (v1)",
    extensions: &["tis"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x53, 0x20, 0x56, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
