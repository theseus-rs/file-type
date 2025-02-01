use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7276404: FileFormat = FileFormat {
    id: 7_276_404,
    puid: "wikidata/7276404",
    name: "REX2",
    extensions: &["rex", "rx2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
