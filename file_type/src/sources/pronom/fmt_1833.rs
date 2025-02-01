use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1833: FileFormat = FileFormat {
    id: 2_685,
    puid: "fmt/1833",
    name: "ArcSoft Album and SlideShow Files for PhotoStudio and PhotoImpression",
    extensions: &["abm", "sld"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
