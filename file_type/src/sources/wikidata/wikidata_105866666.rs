use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866666: FileFormat = FileFormat {
    id: 105_866_666,
    puid: "wikidata/105866666",
    name: "Palm DiddleBug sketch",
    extensions: &["pdb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x49, 0x44, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
