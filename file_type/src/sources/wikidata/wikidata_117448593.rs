use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117448593: FileFormat = FileFormat {
    id: 117_448_593,
    puid: "wikidata/117448593",
    name: "FLExText Interlinear XML Format",
    extensions: &["flextext"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
