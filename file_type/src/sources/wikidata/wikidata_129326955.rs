use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129326955: FileFormat = FileFormat {
    id: 129_326_955,
    puid: "wikidata/129326955",
    name: "GDScript source code file",
    extensions: &["gd", "gd"],
    media_types: &["application/x-gdscript", "text/x-gdscript"],
    internal_signatures: &[],
    related_formats: &[],
};
