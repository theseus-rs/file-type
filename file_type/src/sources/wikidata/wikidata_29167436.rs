use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167436: FileFormat = FileFormat {
    id: 29_167_436,
    puid: "wikidata/29167436",
    name: "Microsoft Object Description Language",
    extensions: &["odl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
