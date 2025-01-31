use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119785986: FileFormat = FileFormat {
    id: 119_785_986,
    puid: "wikidata/119785986",
    name: "MasterCook Calendar File",
    extensions: &["mcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
