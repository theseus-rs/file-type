use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111356350: FileFormat = FileFormat {
    id: 111_356_350,
    puid: "wikidata/111356350",
    name: "Turtle Beach WaveFront program format",
    extensions: &["wfp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
