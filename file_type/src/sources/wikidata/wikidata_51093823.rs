use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51093823: FileFormat = FileFormat {
    id: 51_093_823,
    puid: "wikidata/51093823",
    name: "AutoCAD Plot Configuration File, version R14",
    extensions: &["pc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
