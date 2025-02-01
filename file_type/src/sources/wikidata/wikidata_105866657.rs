use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866657: FileFormat = FileFormat {
    id: 105_866_657,
    puid: "wikidata/105866657",
    name: "Palm PeanutReader e-book",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4E, 0x52, 0x64, 0x50, 0x50, 0x72, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
