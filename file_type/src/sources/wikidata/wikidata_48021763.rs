use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48021763: FileFormat = FileFormat {
    id: 48_021_763,
    puid: "wikidata/48021763",
    name: "Microsoft Access Database, version 2002",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
