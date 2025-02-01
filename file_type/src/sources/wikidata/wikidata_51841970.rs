use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51841970: FileFormat = FileFormat {
    id: 51_841_970,
    puid: "wikidata/51841970",
    name: "Microsoft FoxPro Library",
    extensions: &["plb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
