use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4836515: FileFormat = FileFormat {
    id: 4_836_515,
    puid: "wikidata/4836515",
    name: "BSAVE",
    extensions: &["bsv", "cgx", "pic", "scn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
