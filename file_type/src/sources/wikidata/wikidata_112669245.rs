use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112669245: FileFormat = FileFormat {
    id: 112_669_245,
    puid: "wikidata/112669245",
    name: "Lightscape Layers",
    extensions: &["lay"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
