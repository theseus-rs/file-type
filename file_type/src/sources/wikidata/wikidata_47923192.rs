use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47923192: FileFormat = FileFormat {
    id: 47_923_192,
    puid: "wikidata/47923192",
    name: "Microsoft Word for Macintosh Document, version 5.0",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x37, 0x00, 0x23])],
            },
        }],
    }],
    related_formats: &[],
};
