use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207167: FileFormat = FileFormat {
    id: 28_207_167,
    source_type: SourceType::Wikidata,
    name: "Piecewise-Constant Image Model",
    extensions: &["pwc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0x79, 0x56, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
