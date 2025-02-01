use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63339321: FileFormat = FileFormat {
    id: 63_339_321,
    puid: "wikidata/63339321",
    name: "Microsoft Works Database for Windows, version 4.5a",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
