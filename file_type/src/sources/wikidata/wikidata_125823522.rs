use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125823522: FileFormat = FileFormat {
    id: 125_823_522,
    puid: "wikidata/125823522",
    name: "Microsoft Help Attribute Definition file",
    extensions: &["hxw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
