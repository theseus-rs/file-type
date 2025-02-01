use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114131966: FileFormat = FileFormat {
    id: 114_131_966,
    puid: "wikidata/114131966",
    name: "Chem3D template",
    extensions: &["c3t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
