use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117456796: FileFormat = FileFormat {
    id: 117_456_796,
    puid: "wikidata/117456796",
    name: "Microsoft Access Encrypted Database File 1.0",
    extensions: &["mda", "mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
