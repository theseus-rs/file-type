use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2685: FileFormat = FileFormat {
    id: 2_685,
    source_type: SourceType::Pronom,
    name: "ArcSoft Album and SlideShow Files for PhotoStudio and PhotoImpression",
    extensions: &["abm", "sld"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x4C, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
