use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849925: FileFormat = FileFormat {
    id: 105_849_925,
    source_type: SourceType::Wikidata,
    name: "Windows Cardfile database (with OLE 1.0 objects)",
    extensions: &["crd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x52, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
