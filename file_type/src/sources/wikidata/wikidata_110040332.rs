use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110040332: FileFormat = FileFormat {
    id: 110_040_332,
    puid: "wikidata/110040332",
    name: "Harvard Graphics Presentation, version 1-3 PRS",
    extensions: &["prs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
