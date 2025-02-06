use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856105: FileFormat = FileFormat {
    id: 105_856_105,
    source_type: SourceType::Wikidata,
    name: "DNG Camera Profile",
    extensions: &["dcp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x43, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
