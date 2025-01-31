use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130357981: FileFormat = FileFormat {
    id: 130_357_981,
    puid: "wikidata/130357981",
    name: "MoonScript source code file",
    extensions: &["moon", "moon"],
    media_types: &["application/x-moonscript", "text/x-moonscript"],
    internal_signatures: &[],
    related_formats: &[],
};
