use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72175258: FileFormat = FileFormat {
    id: 72_175_258,
    puid: "wikidata/72175258",
    name: "Kaspersky Anti-Virus signature bases",
    extensions: &["kdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
