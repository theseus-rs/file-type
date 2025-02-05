use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858352: FileFormat = FileFormat {
    id: 105_858_352,
    source_type: SourceType::Wikidata,
    name: "SimCity 4 Cohort (binary)",
    extensions: &["exmp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x51, 0x5A, 0x42, 0x31, 0x23, 0x23, 0x23,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
