use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850597: FileFormat = FileFormat {
    id: 105_850_597,
    source_type: SourceType::Wikidata,
    name: "Borland Client Dataset data",
    extensions: &["cds"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x96, 0x19, 0xE0, 0xBD])],
            },
        }],
    }],
    related_formats: &[],
};
