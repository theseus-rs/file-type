use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73160459: FileFormat = FileFormat {
    id: 73_160_459,
    puid: "wikidata/73160459",
    name: "Visio Workspace",
    extensions: &["vsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
