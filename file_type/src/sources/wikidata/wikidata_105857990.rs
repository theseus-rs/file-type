use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857990: FileFormat = FileFormat {
    id: 105_857_990,
    source_type: SourceType::Wikidata,
    name: "DOSIMG disk image (80t/18s)",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x12, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
