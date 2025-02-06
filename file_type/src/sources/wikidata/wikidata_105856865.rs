use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856865: FileFormat = FileFormat {
    id: 105_856_865,
    source_type: SourceType::Wikidata,
    name: "Adobe Photoshop gradient",
    extensions: &["grd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x38, 0x42, 0x47, 0x52, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
