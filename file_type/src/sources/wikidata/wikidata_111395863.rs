use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111395863: FileFormat = FileFormat {
    id: 111_395_863,
    source_type: SourceType::Wikidata,
    name: "STiNG format",
    extensions: &["stn", "stng"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x4E, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
