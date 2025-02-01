use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1196547: FileFormat = FileFormat {
    id: 1_196_547,
    puid: "wikidata/1196547",
    name: "Design Web Format",
    extensions: &["dwf", "dwfx"],
    media_types: &["model/vnd-dwf", "model/vnd-dwf"],
    internal_signatures: &[],
    related_formats: &[],
};
