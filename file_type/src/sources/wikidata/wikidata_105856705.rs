use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856705: FileFormat = FileFormat {
    id: 105_856_705,
    puid: "wikidata/105856705",
    name: "Universal Voxel format",
    extensions: &["uvox"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x64, 0x73, 0x20, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x5F, 0x74,
                    0x79, 0x70, 0x65, 0x20, 0x75, 0x76, 0x6F, 0x78,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
