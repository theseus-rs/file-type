use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130466597: FileFormat = FileFormat {
    id: 130_466_597,
    puid: "wikidata/130466597",
    name: "Parsing Expression Grammar file format",
    extensions: &["peg"],
    media_types: &["text/x-peg"],
    internal_signatures: &[],
    related_formats: &[],
};
