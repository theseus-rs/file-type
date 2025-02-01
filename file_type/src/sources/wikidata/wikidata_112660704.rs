use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112660704: FileFormat = FileFormat {
    id: 112_660_704,
    puid: "wikidata/112660704",
    name: "Portfolio File",
    extensions: &["bfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
