use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130285087: FileFormat = FileFormat {
    id: 130_285_087,
    puid: "wikidata/130285087",
    name: "Minecraft Add-ons Data Schema File",
    extensions: &["mcschema"],
    media_types: &["text/mcschema"],
    internal_signatures: &[],
    related_formats: &[],
};
