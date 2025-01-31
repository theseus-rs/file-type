use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63339218: FileFormat = FileFormat {
    id: 63_339_218,
    puid: "wikidata/63339218",
    name: "Microsoft Works Database for Windows, version 4.5",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
