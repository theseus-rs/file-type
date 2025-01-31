use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109596500: FileFormat = FileFormat {
    id: 109_596_500,
    puid: "wikidata/109596500",
    name: "DrawPlus Animation",
    extensions: &["dpa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
