use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27355579: FileFormat = FileFormat {
    id: 27_355_579,
    puid: "wikidata/27355579",
    name: "ADRG Overview Image File",
    extensions: &["ovr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
