use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119406880: FileFormat = FileFormat {
    id: 119_406_880,
    puid: "wikidata/119406880",
    name: "ACT! Data File",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
