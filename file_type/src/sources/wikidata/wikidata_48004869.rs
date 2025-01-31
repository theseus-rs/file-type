use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48004869: FileFormat = FileFormat {
    id: 48_004_869,
    puid: "wikidata/48004869",
    name: "Microsoft Access Database, version 95",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
