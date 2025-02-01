use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118669892: FileFormat = FileFormat {
    id: 118_669_892,
    puid: "wikidata/118669892",
    name: "Layer Link File",
    extensions: &["clk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
