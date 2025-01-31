use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122463531: FileFormat = FileFormat {
    id: 122_463_531,
    puid: "wikidata/122463531",
    name: "Microsoft Visual Basic Include file",
    extensions: &["bi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
