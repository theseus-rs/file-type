use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125822754: FileFormat = FileFormat {
    id: 125_822_754,
    puid: "wikidata/125822754",
    name: "Microsoft Help Index",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
