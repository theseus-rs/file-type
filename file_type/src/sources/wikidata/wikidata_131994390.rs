use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131994390: FileFormat = FileFormat {
    id: 131_994_390,
    puid: "wikidata/131994390",
    name: "Web Extracted Text",
    extensions: &["wet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
