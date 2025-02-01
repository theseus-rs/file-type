use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111356337: FileFormat = FileFormat {
    id: 111_356_337,
    puid: "wikidata/111356337",
    name: "Turtle Beach WaveFront drum set format",
    extensions: &["wfd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
