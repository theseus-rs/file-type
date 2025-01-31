use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117456477: FileFormat = FileFormat {
    id: 117_456_477,
    puid: "wikidata/117456477",
    name: "Microsoft Access Database File 1.1",
    extensions: &["mda", "mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
