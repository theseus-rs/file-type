use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117456349: FileFormat = FileFormat {
    id: 117_456_349,
    puid: "wikidata/117456349",
    name: "Microsoft Access Database File 1.0",
    extensions: &["mda", "mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
