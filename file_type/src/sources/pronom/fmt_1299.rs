use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1299: FileFormat = FileFormat {
    id: 2_117,
    puid: "fmt/1299",
    name: "Broderbund Print Shop Deluxe",
    extensions: &[
        "pdb", "pds", "pcb", "pdc", "pcc", "pce", "pdg", "pdl", "pso", "pdp", "pho", "pcp", "ppi",
        "pda",
    ],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x79, 0xA6, 0x00, 0x00, 0x49, 0x49, 0x49, 0x49, 0x38,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 240,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
