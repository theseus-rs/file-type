use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2481: FileFormat = FileFormat {
    id: 2_481,
    source_type: SourceType::Pronom,
    name: "Palm Database ImageViewer Format",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(60),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x49, 0x4D, 0x47, 0x56, 0x69, 0x65, 0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
