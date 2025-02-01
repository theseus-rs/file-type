use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47539022: FileFormat = FileFormat {
    id: 47_539_022,
    puid: "wikidata/47539022",
    name: "AutoCAD Drawing Standards File",
    extensions: &["dws"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
