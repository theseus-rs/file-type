use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128770247: FileFormat = FileFormat {
    id: 128_770_247,
    puid: "wikidata/128770247",
    name: "CFEngine3 file format",
    extensions: &["cf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
