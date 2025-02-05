use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861740: FileFormat = FileFormat {
    id: 105_861_740,
    source_type: SourceType::Wikidata,
    name: "PlayStation RSD Mesh (gen)",
    extensions: &["msh"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x4D, 0x53, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};
