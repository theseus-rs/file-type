use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125823450: FileFormat = FileFormat {
    id: 125_823_450,
    puid: "wikidata/125823450",
    name: "Microsoft Help Data file",
    extensions: &["hxr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
