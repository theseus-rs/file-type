use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858673: FileFormat = FileFormat {
    id: 105_858_673,
    source_type: SourceType::Wikidata,
    name: "MSX compressed Image bitmap",
    extensions: &["mig"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x58, 0x4D, 0x49, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
