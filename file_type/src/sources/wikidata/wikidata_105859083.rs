use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859083: FileFormat = FileFormat {
    id: 105_859_083,
    source_type: SourceType::Wikidata,
    name: "Babylon Dictionary",
    extensions: &["bdc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x12, 0x34, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
