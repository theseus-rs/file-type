use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_68481410: FileFormat = FileFormat {
    id: 68_481_410,
    puid: "wikidata/68481410",
    name: "AutoCAD Sheet Set file format",
    extensions: &["dst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
