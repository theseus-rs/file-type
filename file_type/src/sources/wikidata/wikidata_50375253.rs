use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50375253: FileFormat = FileFormat {
    id: 50_375_253,
    puid: "wikidata/50375253",
    name: "Extensible Metadata Platform Format",
    extensions: &["xmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
