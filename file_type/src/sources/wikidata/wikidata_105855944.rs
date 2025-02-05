use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855944: FileFormat = FileFormat {
    id: 105_855_944,
    source_type: SourceType::Wikidata,
    name: "Embroidery file (generic)",
    extensions: &["dst"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x41, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
