use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_287: FileFormat = FileFormat {
    id: 1_027,
    puid: "fmt/287",
    name: "HDF5",
    extensions: &["hdf5", "h5", "hdf", "nc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x89, 0x48, 0x44, 0x46, 0x0D, 0x0A, 0x1A, 0x0A, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_875,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
