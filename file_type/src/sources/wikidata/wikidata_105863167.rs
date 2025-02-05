use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863167: FileFormat = FileFormat {
    id: 105_863_167,
    source_type: SourceType::Wikidata,
    name: "Wolfpack Mission (from CD)",
    extensions: &["mis"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x64, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
