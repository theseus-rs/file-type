use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855376: FileFormat = FileFormat {
    id: 105_855_376,
    source_type: SourceType::Wikidata,
    name: "F.R.A.C. project",
    extensions: &["file"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x52, 0x41, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
