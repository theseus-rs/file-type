use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857574: FileFormat = FileFormat {
    id: 105_857_574,
    puid: "wikidata/105857574",
    name: "InstallShield Project",
    extensions: &["ipr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
