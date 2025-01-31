use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50376365: FileFormat = FileFormat {
    id: 50_376_365,
    puid: "wikidata/50376365",
    name: "File Geodatabase (Esri)",
    extensions: &["gdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
