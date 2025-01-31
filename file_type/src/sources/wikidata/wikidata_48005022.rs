use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48005022: FileFormat = FileFormat {
    id: 48_005_022,
    puid: "wikidata/48005022",
    name: "Microsoft Access Database, version 97",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
