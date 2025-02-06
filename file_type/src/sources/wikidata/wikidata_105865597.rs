use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865597: FileFormat = FileFormat {
    id: 105_865_597,
    source_type: SourceType::Wikidata,
    name: "Total Project Manager Project",
    extensions: &["prj"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x02, 0x00, 0x04, 0x04, 0x0B, 0x00, 0x18, 0x00, 0x50, 0x52, 0x4F,
                    0x4A, 0x48, 0x45, 0x41, 0x44, 0x00, 0x50, 0x52, 0x4F, 0x4A, 0x44, 0x41, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
