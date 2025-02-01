use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63391705: FileFormat = FileFormat {
    id: 63_391_705,
    puid: "wikidata/63391705",
    name: "Microsoft Works Database for DOS, version 3b",
    extensions: &["wdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
