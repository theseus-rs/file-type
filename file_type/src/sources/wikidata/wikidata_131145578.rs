use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131145578: FileFormat = FileFormat {
    id: 131_145_578,
    puid: "wikidata/131145578",
    name: "Spice source file format",
    extensions: &["spice"],
    media_types: &["text/x-spice"],
    internal_signatures: &[],
    related_formats: &[],
};
