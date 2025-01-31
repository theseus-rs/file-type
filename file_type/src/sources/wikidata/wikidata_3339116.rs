use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3339116: FileFormat = FileFormat {
    id: 3_339_116,
    puid: "wikidata/3339116",
    name: "Newick tree format",
    extensions: &["newick"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
