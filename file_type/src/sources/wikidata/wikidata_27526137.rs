use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27526137: FileFormat = FileFormat {
    id: 27_526_137,
    puid: "wikidata/27526137",
    name: "Microsoft Word for Macintosh Document, version 3.0",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x34, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
