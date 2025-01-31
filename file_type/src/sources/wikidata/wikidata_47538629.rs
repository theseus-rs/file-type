use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538629: FileFormat = FileFormat {
    id: 47_538_629,
    puid: "wikidata/47538629",
    name: "AutoCAD Colour-Dependant Plot Style Table",
    extensions: &["ctb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
