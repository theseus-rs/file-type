use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48004607: FileFormat = FileFormat {
    id: 48_004_607,
    puid: "wikidata/48004607",
    name: "Microsoft Access Database, version 2",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
