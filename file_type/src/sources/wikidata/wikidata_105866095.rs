use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866095: FileFormat = FileFormat {
    id: 105_866_095,
    source_type: SourceType::Wikidata,
    name: "Palm PocketChess deluxe games library",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x41, 0x4D, 0x45, 0x70, 0x58, 0x63, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
