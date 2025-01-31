use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51093854: FileFormat = FileFormat {
    id: 51_093_854,
    puid: "wikidata/51093854",
    name: "AutoCAD Plot Configuration File, version 2000",
    extensions: &["pc3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
